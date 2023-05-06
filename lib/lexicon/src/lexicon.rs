use std::collections::HashMap;

use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use serde_json::Value as JSONValue;

pub type XrpcParameter = String;
pub type XrpcError = String;

pub type JV = JSONValue;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum StringFormat {
    Datetime,
    Uri,
    AtUri,
    Did,
    Handle,
    AtIdentifier,
    Nsid,
    Cid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LexString {
    pub format: Option<StringFormat>,
    pub description: Option<String>,
    pub default: Option<String>,
    pub min_length: Option<u64>,
    pub max_length: Option<u64>,
    pub min_graphemes: Option<u64>,
    pub max_graphemes: Option<u64>,
    pub r#enum: Option<Vec<String>>,
    pub r#const: Option<String>,
    pub known_values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LexBoolean {
    pub description: Option<String>,
    pub default: Option<bool>,
    pub r#const: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LexInteger {
    pub description: Option<String>,
    pub default: Option<i64>,
    pub minimum: Option<i64>,
    pub maximum: Option<i64>,
    pub r#enum: Option<Vec<i64>>,
    pub r#const: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Primitive {
    #[serde(rename = "boolean")]
    Boolean(LexBoolean),
    #[serde(rename = "integer")]
    Integer(LexInteger),
    #[serde(rename = "string")]
    String(LexString),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefUnion {
    pub description: Option<String>,
    pub refs: Vec<String>,
    pub closed: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum RefVariant {
    Ref(Ref),
    Union(RefUnion),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ArrayItem {
    Primitive(Primitive),
    RefVariant(RefVariant),
    Unknown(JV), // TODO
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Array {
    pub description: Option<String>,
    pub items: Box<ArrayItem>,
    pub min_length: Option<u64>,
    pub max_length: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Parameter {
    Primitive(Primitive),
    Array(Array),
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub struct Parameters {
    pub required: Option<Vec<String>>,
    pub properties: Option<HashMap<String, Parameter>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum XrpcBodySchema {
    RefVariant(RefVariant),
    Object(Object),
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct XrpcBody {
    pub encoding: Option<String>,
    pub schema: Option<XrpcBodySchema>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XrpcQuery {
    pub description: Option<String>,
    pub parameters: Option<Parameters>,
    pub output: Option<XrpcBody>,
    pub errors: Option<Vec</* TODO */ JSONValue>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XrpcProcedure {
    pub description: Option<String>,
    pub parameters: Option<Parameters>,
    pub input: Option<XrpcBody>,
    pub output: Option<XrpcBody>,
    pub errors: Option<Vec</* TODO */ JSONValue>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XrpcSubscription {
    pub description: Option<String>,
    pub parameters: Option<Parameters>,
    pub message: Option</* TODO */ JSONValue>,
    pub infos: Option<Vec</* TODO */ JSONValue>>,
    pub errors: Option<Vec</* TODO */ JSONValue>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ref {
    pub description: Option<String>,
    pub r#ref: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blob {
    pub description: Option<String>,
    pub accept: Option<Vec<String>>,
    pub max_size: Option<u64>,
}

#[derive(Serialize, Debug, Clone)]
pub enum ObjectField {
    Primitive(Primitive),
    RefVariant(RefVariant),
    Array(Array),
    Blob(Blob),
    Unknown,
}

impl<'de> Deserialize<'de> for ObjectField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let object: serde_json::Value = Deserialize::deserialize(deserializer)?;
        let r#type = object.get("type").ok_or_else(|| {
            serde::de::Error::custom("missing `type` field in object field definition")
        })?;
        let r#type = r#type
            .as_str()
            .ok_or_else(|| serde::de::Error::custom("`type` field is not a string"))?;
        match r#type {
            "string" | "integer" | "boolean" => Ok(ObjectField::Primitive(
                serde_json::from_value(object).map_err(|_| {
                    serde::de::Error::custom(
                        "failed to deserialize primitive object field definition",
                    )
                })?,
            )),
            "ref" | "union" => Ok(ObjectField::RefVariant(
                serde_json::from_value(object).map_err(|_| {
                    serde::de::Error::custom("failed to deserialize ref object field definition")
                })?,
            )),
            "array" => Ok(ObjectField::Array(serde_json::from_value(object).map_err(
                |_| serde::de::Error::custom("failed to deserialize array object field definition"),
            )?)),
            "blob" => Ok(ObjectField::Blob(serde_json::from_value(object).map_err(
                |_| serde::de::Error::custom("failed to deserialize blob object field definition"),
            )?)),
            "unknown" => Ok(ObjectField::Unknown),
            "cid-link" | "bytes" => {
                println!("cid-link object field type");
                Ok(ObjectField::Unknown)
            }
            v => unimplemented!("Unknown object field type: \"{}\"", v),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Object {
    pub description: Option<String>,
    pub required: Option<Vec<String>>,
    pub nullable: Option<Vec<String>>,
    pub properties: Option<HashMap<String, ObjectField>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Enum {
    pub known_values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Record {
    pub description: Option<String>,
    pub key: Option<String>,
    pub record: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token {
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bytes {
    description: Option<String>,
    min_length: Option<u64>,
    max_length: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CidLink {
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum UserType {
    Record(Record),

    #[serde(rename = "query")]
    XrpcQuery(XrpcQuery),
    #[serde(rename = "procedure")]
    XrpcProcedure(XrpcProcedure),
    #[serde(rename = "subscription")]
    XrpcSubscription(XrpcSubscription),

    Blob(Blob),

    Array(Array),
    Token(Token),
    Object(Object),

    String(LexString),

    Bytes(Bytes),
    CidLink(CidLink),

    #[serde(other)]
    Unknown,
}

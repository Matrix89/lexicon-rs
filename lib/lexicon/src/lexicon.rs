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
#[serde(deny_unknown_fields)]
pub struct Datetime {
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Uri {
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct AtUri {
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Did {
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Handle {
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct AtIdentifier {
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Nsid {
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Cid {
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct OtherString {
    pub description: Option<String>,
    pub known_values: Option<Vec<String>>,
    pub default: Option<String>,
    pub max_graphemes: Option<u64>,
    pub max_length: Option<u64>,
    pub min_length: Option<u64>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum LexString {
    Datetime(Datetime),
    Uri(Uri),
    AtUri(AtUri),
    Did(Did),
    Handle(Handle),
    AtIdentifier(AtIdentifier),
    Nsid(Nsid),
    Cid(Cid),
    OtherString(OtherString),
}

impl<'de> Deserialize<'de> for LexString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut object: serde_json::Map<String, serde_json::Value> =
            Deserialize::deserialize(deserializer)?;
        let format = object.remove("format");
        let object = serde_json::Value::Object(object);
        match format {
            Some(v) => match v.as_str().unwrap() {
                "did" => Ok(LexString::Did(serde_json::from_value(object).map_err(
                    |err| {
                        println!("did {:?}", err);
                        serde::de::Error::custom(format!(
                            "failed to deserialize did definition, err: {:?}",
                            err
                        ))
                    },
                )?)),
                "handle" => Ok(LexString::Handle(serde_json::from_value(object).map_err(
                    |err| {
                        println!("handle {:?}", err);
                        serde::de::Error::custom(format!(
                            "failed to deserialize handle definition, err: {:?}",
                            err
                        ))
                    },
                )?)),
                "datetime" => Ok(LexString::Datetime(
                    serde_json::from_value(object).map_err(|err| {
                        println!("datetime {:?}", err);
                        serde::de::Error::custom(format!(
                            "failed to deserialize datetime definition, err: {:?}",
                            err
                        ))
                    })?,
                )),
                "at-uri" => Ok(LexString::AtUri(serde_json::from_value(object).map_err(
                    |err| {
                        println!("at-uri {:?}", err);
                        serde::de::Error::custom(format!(
                            "failed to deserialize at-uri definition, err: {:?}",
                            err
                        ))
                    },
                )?)),
                "at-identifier" => Ok(LexString::AtIdentifier(
                    serde_json::from_value(object).map_err(|err| {
                        println!("at-identifier {:?}", err);
                        serde::de::Error::custom(format!(
                            "failed to deserialize at-identifier definition, err: {:?}",
                            err
                        ))
                    })?,
                )),
                "nsid" => Ok(LexString::Nsid(serde_json::from_value(object).map_err(
                    |err| {
                        println!("nsid {:?}", err);
                        serde::de::Error::custom(format!(
                            "failed to deserialize nsid definition, err: {:?}",
                            err
                        ))
                    },
                )?)),
                "uri" => Ok(LexString::Uri(serde_json::from_value(object).map_err(
                    |err| {
                        println!("uri {:?}", err);
                        serde::de::Error::custom(format!(
                            "failed to deserialize uri definition, err: {:?}",
                            err
                        ))
                    },
                )?)),
                "cid" => Ok(LexString::Cid(serde_json::from_value(object).map_err(
                    |err| {
                        println!("cid {:?}", err);
                        serde::de::Error::custom(format!(
                            "failed to deserialize cid definition, err: {:?}",
                            err
                        ))
                    },
                )?)),
                v => {
                    println!("unknown format: {:?}", v);
                    Err(serde::de::Error::custom(format!("unknown format: {:?}", v)))
                }
            },
            None => Ok(LexString::OtherString(
                serde_json::from_value(object).map_err(|err| {
                    println!("other {:?}", err);
                    serde::de::Error::custom(format!(
                        "failed to deserialize string definition, err: {}",
                        err
                    ))
                })?,
            )),
        }
    }
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
    #[serde(default)]
    pub required: Vec<String>,
    #[serde(default)]
    pub properties: HashMap<String, Parameter>,
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
    #[serde(default)]
    pub parameters: Parameters,
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
                serde_json::from_value(object).map_err(|err| {
                    serde::de::Error::custom(
                        format!("failed to deserialize primitive object field definition, inner error: {:?}", err),
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

use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};
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
    Boolean(LexBoolean),
    #[serde(rename = "integer")]
    Integer(LexInteger),
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
    Ref(Ref),
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ObjectField {
    Primitive(Primitive),
    RefVariant(RefVariant),
    Array(Array),
    Blob(Blob),
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
#[serde(tag = "type", rename_all = "camelCase")]
pub enum UserType {
    Record {
        description: Option<String>,
        key: Option<String>,
        record: Box<UserType>,
    },
    #[serde(rename = "query")]
    XrpcQuery(XrpcQuery),
    #[serde(rename = "procedure")]
    XrpcProcedure(XrpcProcedure),
    #[serde(rename = "subscription")]
    XrpcSubscription(XrpcSubscription),

    Blob(Blob),

    Array(Array),
    Token {
        description: Option<String>,
    },
    Object(Object),
    Union(RefVariant),
    Ref(Ref),
    Bytes {
        description: Option<String>,
        min_length: Option<u64>,
        max_length: Option<u64>,
    },
    CidLink {
        description: Option<String>,
    },

    Primitive(Primitive),

    Enum(Enum),

    #[serde(other)]
    Unknown,
}

// https://atproto.com/specs/lexicon#interface
#[derive(Serialize, Deserialize, Debug)]
pub struct LexiconDoc {
    pub lexicon: u8,
    pub id: String,

    pub revision: Option<u8>,
    pub description: Option<String>,

    pub defs: HashMap<String, UserType>,
}

impl FromStr for LexiconDoc {
    type Err = serde_path_to_error::Error<serde_json::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let deserializer = &mut serde_json::Deserializer::from_str(s);
        serde_path_to_error::deserialize(deserializer)
    }
}

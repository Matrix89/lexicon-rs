use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_json::Value as JSONValue;

pub type XrpcParameter = String;
pub type XrpcBody = String;
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

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub struct Parameters {
    pub required: Option<Vec<String>>,
    pub properties: Option<HashMap<String, UserType>>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Output {
    pub encoding: Option<String>,
    pub schema: Option<Box<UserType>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum UserType {
    Record {
        description: Option<String>,
        key: Option<String>,
        record: HashMap<String, JSONValue>,
    },

    #[serde(rename = "query")]
    XrpcQuery {
        description: Option<String>,
        parameters: Option<Parameters>,
        output: Option<Output>,
        errors: Option<Vec</* TODO */ JSONValue>>,
    },
    #[serde(rename = "procedure")]
    XrpcProcedure {
        description: Option<String>,
        parameters: Option<Parameters>,
        input: Option</* TODO */ JSONValue>,
        output: Option<Output>,
        errors: Option<Vec</* TODO */ JSONValue>>,
    },
    #[serde(rename = "subscription")]
    XrpcSubscription {
        description: Option<String>,
        parameters: Option<Parameters>,
        message: Option</* TODO */ JSONValue>,
        infos: Option<Vec</* TODO */ JSONValue>>,
        errors: Option<Vec</* TODO */ JSONValue>>,
    },

    Blob {
        description: Option<String>,
        accept: Option<Vec<String>>,
        max_size: Option<u64>,
    },

    Array {
        description: Option<String>,
        items: Box<UserType>,
        min_length: Option<u64>,
        max_length: Option<u64>,
    },
    Token {
        description: Option<String>,
    },
    Object {
        description: Option<String>,
        required: Option<Vec<String>>,
        nullable: Option<Vec<String>>,
        properties: Option<HashMap<String, UserType>>,
    },
    Union {
        description: Option<String>,
        refs: Vec<String>,
        closed: Option<bool>,
    },
    Boolean {
        description: Option<String>,
        default: Option<bool>,
        r#const: Option<bool>,
    },
    Integer {
        description: Option<String>,
        default: Option<i64>,
        minimum: Option<i64>,
        maximum: Option<i64>,
        r#enum: Option<Vec<i64>>,
        r#const: Option<i64>,
    },
    Ref {
        description: Option<String>,
        r#ref: String,
    },
    #[serde(rename_all = "camelCase")]
    String {
        format: Option<StringFormat>,
        description: Option<String>,
        default: Option<String>,
        min_length: Option<u64>,
        max_length: Option<u64>,
        min_graphemes: Option<u64>,
        max_graphemes: Option<u64>,
        r#enum: Option<Vec<String>>,
        r#const: Option<String>,
        known_values: Option<Vec<String>>,
    },
    Bytes {
        description: Option<String>,
        min_length: Option<u64>,
        max_length: Option<u64>,
    },
    CidLink {
        description: Option<String>,
    },

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
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

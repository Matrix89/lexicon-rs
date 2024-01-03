use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::lexicon::UserType;

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

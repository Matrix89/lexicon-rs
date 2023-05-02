use quote::quote;
use std::{collections::HashMap, fmt};

use proc_macro2::TokenStream;

pub struct DocBuilder {
    items: HashMap<String, String>,
}

impl DocBuilder {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn add_optional_item(&mut self, name: &str, value: &Option<impl fmt::Debug>) {
        if let Some(value) = value {
            self.items.insert(name.to_string(), format!("{:?}", value));
        }
    }

    pub fn add_item(&mut self, name: &str, value: impl fmt::Debug) {
        self.items.insert(name.to_string(), format!("{:?}", value));
    }

    pub fn build(&self) -> TokenStream {
        self.items
            .iter()
            .map(|(name, value)| {
                let value = format!("{}: {}", name, value);
                quote! {
                    #[doc = #value]
                }
            })
            .collect::<TokenStream>()
    }
}

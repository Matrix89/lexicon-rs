use quote::quote;
use std::{collections::HashMap, fmt};

use proc_macro2::TokenStream;

pub struct DocBuilder {
    items: HashMap<String, String>,
    arguments: HashMap<String, String>,
}

impl DocBuilder {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            arguments: HashMap::new(),
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

    pub fn add_argument(&mut self, name: &str, value: Option<impl fmt::Debug>) {
        if let Some(value) = value {
            self.arguments
                .insert(name.to_string(), format!("{:?}", value));
        }
    }

    pub fn build(&self) -> TokenStream {
        let props = self
            .items
            .iter()
            .map(|(name, value)| {
                let value = format!("{}: {}", name, value);
                quote! {
                    #[doc = #value]
                }
            })
            .collect::<TokenStream>();

        let arguments = {
            if self.arguments.is_empty() {
                return quote! {};
            }

            let arguments = self
                .arguments
                .iter()
                .map(|(name, value)| {
                    let value = format!("* `{}` - {}", name, value);
                    quote! {
                        #[doc = #value]
                    }
                })
                .collect::<TokenStream>();
            quote! {
                #[doc = "# Arguments"]
                #arguments
            }
        };

        quote! {
            #props
            #arguments
        }
    }
}

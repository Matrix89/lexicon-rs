use std::collections::HashMap;

use convert_case::{Case, Casing};
use lexicon::lexicon::{Parameters, Property};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn gen_parameters(parameters: Parameters) -> TokenStream {
    let properties = parameters.properties.unwrap_or_default();
    let required = parameters.required.unwrap_or_default();

    properties
        .into_iter()
        .map(|(name, value)| {
            let name = format_ident!("{}", name.to_case(Case::Snake));
            match value {
                Property::String {
                    description,
                    format,
                    ..
                } => quote! { #name: String, },
                Property::Array { items, .. } => quote! { #name: Vec<String>, },
                Property::Integer { .. } => quote! { #name: i64, },
                Property::Boolean { .. } => quote! { #name: bool, },
                v => todo!("{:?}", v),
            }
        })
        .collect()
}

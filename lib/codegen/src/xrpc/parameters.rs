use std::collections::HashMap;

use convert_case::{Case, Casing};
use lexicon::lexicon::{Parameters, UserType};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn gen_parameters(parameters: Parameters) -> TokenStream {
    let properties = parameters.properties.unwrap_or_default();
    let required = parameters.required.unwrap_or_default();

    let mut properties = properties.into_iter().collect::<Vec<_>>();
    properties.sort_by(|a, b| a.0.cmp(&b.0));

    properties
        .into_iter()
        .map(|(name, value)| {
            let name = format_ident!("{}", name.to_case(Case::Snake));
            match value {
                UserType::String {
                    description,
                    format,
                    ..
                } => quote! { #name: String, },
                UserType::Array { items, .. } => quote! { #name: Vec<String>, },
                UserType::Integer { .. } => quote! { #name: i64, },
                UserType::Boolean { .. } => quote! { #name: bool, },
                v => todo!("{:?}", v),
            }
        })
        .collect()
}

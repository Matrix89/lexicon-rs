use convert_case::{Case, Casing};
use lexicon::lexicon::{Parameters, Property};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn gen_parameters(parameters: Parameters) -> TokenStream {
    if parameters.properties.is_none() {
        return quote! {};
    }

    parameters
        .properties
        .unwrap()
        .into_iter()
        .map(|(name, value)| {
            let name = format_ident!("{}", name.to_case(Case::Snake));
            match value {
                Property::String {
                    description,
                    format,
                    ..
                } => quote! { #name: String, },
                Property::Array { items } => quote! { #name: Vec<String>, },
                Property::Integer => quote! { #name: i64, },
                Property::Boolean => quote! { #name: bool, },
                v => todo!("{:?}", v),
            }
        })
        .collect()
}

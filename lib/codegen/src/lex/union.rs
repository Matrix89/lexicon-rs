use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::lex::object::build_ref_target;

pub enum KnownValueKind {
    Namespace { namespace: String, ident: String },
    Local { ident: String },
    Literal { value: String },
}

pub fn gen_union(name: &str, refs: Vec<String>, ns: &str) -> (Ident, TokenStream) {
    let values = refs
        .iter()
        .map(|value| {
            if value.contains('.') || value.starts_with('#') {
                let name = value.split('.').last().unwrap().replace('#', " ");
                let name = format_ident!("{}", name.to_case(Case::Pascal));
                let ident = build_ref_target(value);
                let alias = if value.contains('.') {
                    value.to_owned()
                } else {
                    format!(
                        "{}{}",
                        ns.replace("::lexicon::", "").replace("::", "."),
                        value
                    )
                };
                quote! {
                    #[serde(alias = #alias)]
                    #name(Box<#ident>)
                }
            } else {
                let ident = format_ident!("{}", value.to_case(Case::Pascal));
                quote! {
                    #ident
                }
            }
        })
        .collect::<Vec<_>>();

    let enum_name = format_ident!("{}", name.to_case(Case::Pascal));
    let r#enum = quote! {
       #[derive(Clone, Debug, PartialEq, Eq, ::serde::Serialize, ::serde::Deserialize)]
       #[serde(tag = "$type")]
       pub enum #enum_name {
           #(#values),*
       }
    };

    (enum_name, r#enum)
}

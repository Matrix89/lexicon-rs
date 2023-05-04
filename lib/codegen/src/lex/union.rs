use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

pub enum KnownValueKind {
    Namespace { namespace: String, ident: String },
    Local { ident: String },
    Literal { value: String },
}

pub fn gen_union(name: &str, refs: Vec<String>) -> (Ident, TokenStream) {
    let values = refs
        .iter()
        .map(|value| {
            if value.contains('.') {
                let parts = value.split('#').collect::<Vec<_>>();
                let namespace = parts.first().unwrap().to_string();
                let ident = parts.get(1).unwrap_or(&"Main").to_string();
                KnownValueKind::Namespace { namespace, ident }
            } else if value.starts_with('#') {
                KnownValueKind::Local {
                    ident: value.replace('#', "").to_string(),
                }
            } else {
                KnownValueKind::Literal {
                    value: value.to_owned(),
                }
            }
        })
        .map(|value| match value {
            KnownValueKind::Local { ident } => {
                let ident = format_ident!("{}", ident.to_case(Case::Pascal));
                quote! {
                    #ident(Box<#ident>)
                }
            }
            KnownValueKind::Literal { value } => {
                let ident = format_ident!("{}", value.to_case(Case::Pascal));
                quote! {
                    #ident
                }
            }
            KnownValueKind::Namespace { namespace, ident } => {
                let ns_prefix = namespace.split('.').last().unwrap();
                let ident = format_ident!(
                    "{}{}",
                    ns_prefix.to_case(Case::Pascal),
                    ident.to_case(Case::Pascal)
                );
                quote! {
                    #ident
                }
            }
        })
        .collect::<Vec<_>>();

    let enum_name = format_ident!("{}", name.to_case(Case::Pascal));
    let r#enum = quote! {
       #[derive(Clone, Debug, PartialEq, Eq, ::serde::Serialize, ::serde::Deserialize)]
       pub enum #enum_name {
           #(#values),*
       }
    };

    (enum_name, r#enum)
}

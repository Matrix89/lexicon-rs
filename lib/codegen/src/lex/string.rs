use convert_case::{Case, Casing};
use lexicon::lexicon::{LexString, StringFormat};
use nsid::NSIDNode;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::lex::union::KnownValueKind;

pub fn gen_string(
    name: &String,
    tree: &NSIDNode,
    namespace: &String,
    str: &LexString,
) -> TokenStream {
    let name = format_ident!("{}", name.to_case(Case::Pascal));
    match &str.known_values {
        Some(values) => {
            //let (union, union_impl) = gen_union(name, values);
            let values = values
                .iter()
                .map(|value| {
                    if value.contains('.') {
                        let parts = value.split('#').collect::<Vec<_>>();
                        assert!(parts.len() == 2);
                        let namespace = parts.first().unwrap().to_string();
                        let ident = parts.last().unwrap_or(&"Main").to_string();
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
                            #ident
                        }
                    }
                    KnownValueKind::Literal { value } => {
                        let ident = format_ident!("{}", value.to_case(Case::Pascal));
                        quote! {
                            #ident
                        }
                    }
                    KnownValueKind::Namespace { namespace, ident } => {
                        let ident = format_ident!("{}", ident.to_case(Case::Pascal));
                        quote! {
                            #ident
                        }
                    }
                })
                .collect::<Vec<_>>();

            quote! {
            #[derive(Clone, Debug, PartialEq, Eq, ::serde::Serialize, ::serde::Deserialize)]
               pub enum #name {
                   #(#values),*
               }
            }
        }
        None => panic!(
            "TODO! How do we handle top-level strings? {} {:?}",
            namespace, name
        ),
    }
}

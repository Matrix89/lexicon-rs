use convert_case::{Case, Casing};
use lexicon::lexicon::{ArrayItem, Parameter, Parameters, Primitive};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::CodeGen;

impl CodeGen {
    pub fn gen_parameters(&self, parameters: &Parameters) -> TokenStream {
        let properties = parameters.properties.clone().unwrap_or_default();
        let required = parameters.required.clone().unwrap_or_default();

        let mut properties = properties.into_iter().collect::<Vec<_>>();
        properties.sort_by(|a, b| a.0.cmp(&b.0));

        properties
            .into_iter()
            .map(|(name, value)| {
                let name = format_ident!("{}", name.to_case(Case::Snake));
                match value {
                    Parameter::Primitive(primitive) => match primitive {
                        Primitive::String(_) => quote! { #name: String, },
                        Primitive::Integer(_) => quote! { #name: i64, },
                        Primitive::Boolean(_) => quote! { #name:bool, },
                    },
                    Parameter::Array(arr) => {
                        let r#type = match *arr.items {
                            ArrayItem::Primitive(primitive) => match primitive {
                                Primitive::String(_) => quote! { String },
                                Primitive::Integer(_) => quote! { i64 },
                                v => todo!("{:?}", v),
                            },
                            t => todo!("{:?}", t),
                        };

                        quote! { #name: Vec<#r#type>, }
                    }
                }
            })
            .collect()
    }
}

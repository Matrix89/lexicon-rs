use convert_case::{Case, Casing};
use lexicon::lexicon::{ArrayItem, ObjectField, Primitive, UserType};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashMap;

use crate::{doc_builder::DocBuilder, CodeGen};

use super::{array::gen_array, r#ref::gen_ref_variant, string::gen_string, union::gen_union};

pub fn build_ref_target(r#ref: &str) -> syn::Path {
    let ref_target = if r#ref.starts_with('#') {
        r#ref.replace('#', "").to_case(Case::Pascal)
    } else {
        let parts = r#ref.split('#').collect::<Vec<_>>();
        let ns = parts.first().unwrap();
        let ns = ns
            .split('.')
            .map(|seg| seg.to_case(Case::Snake))
            .collect::<Vec<_>>()
            .join("::");
        let ident = parts.get(1).unwrap_or(&"Main");
        let ident = ident.to_case(Case::Pascal);
        format!("lexicon::{}::{}", ns, ident)
    };
    let ref_target: syn::Path = syn::parse_str(ref_target.as_str()).unwrap();
    ref_target
}

pub fn gen_field_name(name: &str) -> Ident {
    let name = name.to_case(Case::Snake);
    let name = match name.as_str() {
        "type" => format!("r#{}", name),
        v => String::from(v),
    };

    format_ident!("{}", name)
}

impl CodeGen {
    fn gen_property(
        &self,
        name: &String,
        namespace: &String,
        object_name: &String,
        property: ObjectField,
        is_required: bool,
        is_nullable: bool,
    ) -> (TokenStream, Option<TokenStream>) {
        let mut doc = DocBuilder::new();

        let (name, prop_type, additional_code) = match property {
            ObjectField::Primitive(primitive) => match primitive {
                Primitive::String(str) => {
                    doc.add_optional_item("description", &str.description);
                    doc.add_optional_item("format", &str.format);
                    doc.add_optional_item("default", &str.default);
                    doc.add_optional_item("min_length", &str.min_length);
                    doc.add_optional_item("max_length", &str.max_length);
                    doc.add_optional_item("min_graphemes", &str.min_graphemes);
                    doc.add_optional_item("max_graphemes", &str.max_graphemes);
                    doc.add_optional_item("enum", &str.r#enum);
                    doc.add_optional_item("const", &str.r#const);
                    doc.add_optional_item("known_values", &str.known_values);

                    match str.known_values {
                        Some(ref know_values) => {
                            let r = gen_string(name, &self.tree, namespace, &str);
                            let name = gen_field_name(name);
                            let prop_type = quote! { String };
                            (name, prop_type, Some(r))
                        }
                        None => {
                            let name = gen_field_name(name);
                            let prop_type = quote! { String };
                            (name, prop_type, None)
                        }
                    }
                }
                Primitive::Boolean(boolean) => {
                    doc.add_optional_item("description", &boolean.description);
                    doc.add_optional_item("default", &boolean.default);
                    doc.add_optional_item("const", &boolean.r#const);

                    let name = gen_field_name(name);
                    (name, quote! { bool }, None)
                }
                Primitive::Integer(int) => {
                    doc.add_optional_item("description", &int.description);
                    doc.add_optional_item("default", &int.default);
                    doc.add_optional_item("minimum", &int.minimum);
                    doc.add_optional_item("maximum", &int.maximum);
                    doc.add_optional_item("enum", &int.r#enum);
                    doc.add_optional_item("const", &int.r#const);

                    let name = gen_field_name(name);

                    (name, quote! { i64 }, None)
                }
            },
            ObjectField::RefVariant(variant) => {
                let (enum_name, r#enum) = gen_ref_variant(&name, variant);
                let name = gen_field_name(name);
                (name, enum_name, r#enum)
            }
            ObjectField::Blob(blob) => {
                doc.add_optional_item("description", &blob.description);
                doc.add_optional_item("accept", &blob.accept);
                doc.add_optional_item("max_size", &blob.max_size);
                let name = format_ident!("{}", name);
                (name, quote! { String }, None)
            }
            ObjectField::Array(array) => {
                doc.add_optional_item("description", &array.description);
                doc.add_optional_item("min_length", &array.min_length);
                doc.add_optional_item("max_length", &array.max_length);

                gen_array(name, array)
            }
            _ => todo!("{:?}", property),
        };

        let prop_type = if is_nullable {
            quote! { Option<#prop_type> }
        } else {
            prop_type
        };

        doc.add_item("nullable", is_nullable);
        doc.add_item("required", is_required);

        let doc = doc.build();

        let property_code = quote! {
            //#doc
            pub #name: #prop_type,
        };

        (property_code, additional_code)
    }

    pub fn gen_object(
        &self,
        object_name: &String,
        namespace: &String,
        description: String,
        required: Vec<String>,
        nullable: Vec<String>,
        properties: HashMap<String, ObjectField>,
    ) -> TokenStream {
        let mut properties_code = vec![];
        let mut additional_declarations = vec![];

        for (name, property) in properties {
            let is_required = required.contains(&name);
            let is_nullable = nullable.contains(&name);
            let (property_code, additional_code) = self.gen_property(
                &name,
                namespace,
                object_name,
                property,
                is_required,
                is_nullable,
            );

            properties_code.push(property_code);
            additional_declarations.push(additional_code);
        }

        let name = format_ident!("{}", object_name.to_case(Case::Pascal));

        quote! {
            //#[doc = #description]
            #(#additional_declarations)*
            #[derive(Clone, Debug, PartialEq, Eq, ::serde::Serialize, ::serde::Deserialize)]
            pub struct #name {
                #(#properties_code)*
            }
        }
    }
}

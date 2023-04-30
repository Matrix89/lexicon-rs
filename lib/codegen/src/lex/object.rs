use convert_case::{Case, Casing};
use lexicon::lexicon::UserType;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashMap;

use crate::{doc_builder::DocBuilder, CodeGen};

use super::{string::gen_string, union::gen_union};

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

fn gen_field_name(name: &str) -> Ident {
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
        property: UserType,
        is_required: bool,
        is_nullable: bool,
    ) -> (TokenStream, Option<TokenStream>) {
        let mut doc = DocBuilder::new();

        let (name, prop_type, additional_code) = match property {
            UserType::String {
                description,
                format,
                default,
                min_length,
                max_length,
                min_graphemes,
                max_graphemes,
                r#enum,
                r#const,
                known_values,
            } => {
                doc.add_optional_item("description", description.clone());
                doc.add_optional_item("format", format);
                doc.add_optional_item("default", default.clone());
                doc.add_optional_item("min_length", min_length);
                doc.add_optional_item("max_length", max_length);
                doc.add_optional_item("min_graphemes", min_graphemes);
                doc.add_optional_item("max_graphemes", max_graphemes);
                doc.add_optional_item("enum", r#enum.clone());
                doc.add_optional_item("const", r#const.clone());
                doc.add_optional_item("known_values", known_values.clone());

                match known_values {
                    Some(ref know_values) => {
                        let r = gen_string(
                            name,
                            &self.tree,
                            namespace,
                            None,
                            description,
                            default,
                            min_length,
                            max_length,
                            min_graphemes,
                            max_graphemes,
                            r#enum,
                            r#const,
                            known_values,
                        );
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
            UserType::Ref { description, r#ref } => {
                doc.add_optional_item("description", description);

                let name = gen_field_name(name);
                let ref_target = build_ref_target(&r#ref);
                (name, quote! { #ref_target }, None)
            }
            UserType::Boolean {
                description,
                default,
                r#const,
            } => {
                doc.add_optional_item("description", description);
                doc.add_optional_item("default", default);
                doc.add_optional_item("const", r#const);

                let name = gen_field_name(name);
                (name, quote! { bool }, None)
            }
            UserType::Array {
                description,
                items,
                min_length,
                max_length,
            } => {
                doc.add_optional_item("description", description);
                doc.add_optional_item("min_length", min_length);
                doc.add_optional_item("max_length", max_length);
                let (array_type, additional_code) = match *items {
                    UserType::Ref { ref r#ref, .. } => {
                        let ref_target = build_ref_target(&r#ref);
                        (quote! { #ref_target }, None)
                    }
                    UserType::String { .. } => (quote! { String }, None),
                    UserType::Integer { .. } => (quote! { i64 }, None),
                    UserType::Union { refs, .. } => {
                        let (enum_name, r#enum) = gen_union(name, refs);
                        (quote! { #enum_name }, Some(r#enum))
                    }
                    UserType::Unknown { .. } => (quote! { String }, None),
                    UserType::CidLink { .. } => (quote! { String }, None),
                    v => todo!("{:?}", v),
                };
                let name = gen_field_name(name);

                let r#type = quote! { Vec<#array_type> };
                (name, r#type, additional_code)
            }
            UserType::Integer {
                description,
                default,
                minimum,
                maximum,
                r#enum,
                r#const,
            } => {
                doc.add_optional_item("description", description);
                doc.add_optional_item("default", default);
                doc.add_optional_item("minimum", minimum);
                doc.add_optional_item("maximum", maximum);
                doc.add_optional_item("enum", r#enum);
                doc.add_optional_item("const", r#const);

                let name = gen_field_name(name);

                (name, quote! { i64 }, None)
            }
            UserType::Blob {
                description,
                accept,
                max_size,
            } => {
                doc.add_optional_item("description", description);
                doc.add_optional_item("accept", accept);
                doc.add_optional_item("max_size", max_size);
                let name = format_ident!("{}", name);
                (name, quote! { String }, None)
            }
            UserType::Union {
                description,
                refs,
                closed,
            } => {
                doc.add_optional_item("description", description);
                doc.add_optional_item("closed", closed);
                let (enum_name, r#enum) = gen_union(&(object_name.to_owned() + &name), refs);
                let name = format_ident!("{}", name);
                (name, quote! { #enum_name }, Some(r#enum))
            }
            UserType::Unknown => {
                //doc.add_optional_item("description", description);

                let name = format_ident!("todo4_{}", name);
                (name, quote! { String }, None)
            }
            UserType::CidLink { description } => {
                doc.add_optional_item("description", description);

                let name = format_ident!("todo3_{}", name);
                (name, quote! { String }, None)
            }
            UserType::Bytes {
                description,
                min_length,
                max_length,
            } => {
                doc.add_optional_item("description", description);
                doc.add_optional_item("min_length", min_length);
                doc.add_optional_item("max_length", max_length);

                let name = format_ident!("todo5_{}", name);
                (name, quote! { String }, None)
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
        properties: HashMap<String, UserType>,
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

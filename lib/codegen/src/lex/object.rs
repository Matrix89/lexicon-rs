use convert_case::{Case, Casing};
use lexicon::lexicon::Property;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashMap;

use crate::{doc_builder::DocBuilder, lex::string::KnownValueKind, CodeGen};

use super::string::gen_string;

pub fn build_ref_target(r#ref: &str) -> syn::Path {
    let ref_target = if r#ref.starts_with('#') {
        r#ref.replace('#', "")
    } else if r#ref.contains('#') {
        format!(
            "crate::lexicon::{}",
            r#ref.replace('.', "::").replace('#', "::")
        )
    } else {
        format!("crate::lexicon::{}::main", r#ref.replace('.', "::"))
    };
    let ref_target = ref_target
        .split("::")
        .map(|segment| {
            if segment != "crate" {
                segment.to_case(Case::Pascal)
            } else {
                segment.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("::");
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

    let enum_name = format_ident!("{}Type", name.to_case(Case::Pascal));
    let r#enum = quote! {
       pub enum #enum_name {
           #(#values),*
       }
    };

    (enum_name, r#enum)
}

impl CodeGen {
    fn gen_property(
        &self,
        name: &String,
        namespace: &String,
        object_name: &String,
        property: Property,
        is_required: bool,
        is_nullable: bool,
    ) -> (TokenStream, Option<TokenStream>) {
        let mut doc = DocBuilder::new();

        let (name, prop_type, additional_code) = match property {
            Property::String {
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
            Property::Ref { description, r#ref } => {
                doc.add_optional_item("description", description);

                let name = gen_field_name(name);
                let ref_target = build_ref_target(&r#ref);
                (name, quote! { #ref_target }, None)
            }
            Property::Boolean {
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
            Property::Array {
                description,
                items,
                min_length,
                max_length,
            } => {
                doc.add_optional_item("description", description);
                doc.add_optional_item("min_length", min_length);
                doc.add_optional_item("max_length", max_length);
                let (array_type, additional_code) = match *items {
                    Property::Ref { ref r#ref, .. } => {
                        let ref_target = build_ref_target(&r#ref);
                        (quote! { #ref_target }, None)
                    }
                    Property::String { .. } => (quote! { String }, None),
                    Property::Integer { .. } => (quote! { i64 }, None),
                    Property::Union { refs, .. } => {
                        let (enum_name, r#enum) = gen_union(name, refs);
                        (quote! { #enum_name }, Some(r#enum))
                    }
                    Property::Unknown { .. } => (quote! { String }, None),
                    Property::CidLink { .. } => (quote! { String }, None),
                    v => todo!("{:?}", v),
                };
                let name = gen_field_name(name);

                let r#type = quote! { Vec<#array_type> };
                (name, r#type, additional_code)
            }
            Property::Integer {
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

                let name = format_ident!("{}", name);

                (name, quote! { i64 }, None)
            }
            Property::Blob {
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
            Property::Union {
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
            Property::Unknown { description } => {
                doc.add_optional_item("description", description);

                let name = format_ident!("todo4_{}", name);
                (name, quote! { String }, None)
            }
            Property::CidLink { description } => {
                doc.add_optional_item("description", description);

                let name = format_ident!("todo3_{}", name);
                (name, quote! { String }, None)
            }
            Property::Bytes {
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
            #doc
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
        properties: HashMap<String, Property>,
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
            #[doc = #description]
            #(#additional_declarations)*
            pub struct #name {
                #(#properties_code)*
            }
        }
    }
}

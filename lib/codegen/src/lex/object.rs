use convert_case::{Case, Casing};
use lexicon::lexicon::Property;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;

use crate::doc_builder::DocBuilder;

fn build_ref_target(r#ref: &str) -> syn::Path {
    let ref_target = if r#ref.starts_with('#') {
        r#ref.replace("#", "").to_string()
    } else {
        format!("lexicon::{}", r#ref.replace('.', "::").replace('#', "::"))
    };
    let ref_target = ref_target
        .split("::")
        .map(|segment| segment.to_case(Case::Pascal))
        .collect::<Vec<_>>()
        .join("::");
    let ref_target: syn::Path = syn::parse_str(ref_target.as_str()).unwrap();
    ref_target
}

fn gen_property(
    name: &String,
    property: Property,
    is_required: bool,
    is_nullable: bool,
) -> (TokenStream, Option<TokenStream>) {
    let name = name.to_case(Case::Snake);

    let is_reserved = name == "type";
    let name = if is_reserved {
        format_ident!("r#{}", name)
    } else {
        format_ident!("{}", name)
    };

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
            doc.add_optional_item("description", description);
            doc.add_optional_item("format", format);
            doc.add_optional_item("default", default);
            doc.add_optional_item("min_length", min_length);
            doc.add_optional_item("max_length", max_length);
            doc.add_optional_item("min_graphemes", min_graphemes);
            doc.add_optional_item("max_graphemes", max_graphemes);
            doc.add_optional_item("enum", r#enum);
            doc.add_optional_item("const", r#const);
            doc.add_optional_item("known_values", known_values);

            let prop_type = quote! { String };
            (name, prop_type, None)
        }
        Property::Ref { description, r#ref } => {
            doc.add_optional_item("description", description);
            let ref_target = build_ref_target(&r#ref);
            (name, quote! { #ref_target }, None)
        }
        Property::Boolean => (name, quote! { bool }, None),
        Property::Array { items } => {
            let name = format_ident!("{}", name);

            let r = match *items {
                Property::Ref { ref r#ref, .. } => quote! { build_ref_target(&r#ref) },
                Property::Union { .. } => quote! { String },
                Property::Unknown { .. } => quote! { String },
                Property::String {
                    format,
                    description,
                    default,
                    min_length,
                    max_length,
                    min_graphemes,
                    max_graphemes,
                    r#enum,
                    r#const,
                    known_values,
                } => quote! { String },
                Property::Integer => quote! { i64 },
                Property::CidLink => quote! { String },
                v => todo!("{:?}", v),
            };

            let r#type = quote! { Vec<#r> };
            (name, r#type, Some(quote! {}))
        }
        Property::Integer => (name, quote! { i64 }, None),
        Property::Blob => {
            let name = format_ident!("todo2_{}", name);
            (name, quote! { String }, None)
        }
        Property::Union => {
            let name = format_ident!("todo3_{}", name);
            (name, quote! { String }, None)
        }
        Property::Unknown => {
            let name = format_ident!("todo4_{}", name);
            (name, quote! { String }, None)
        }
        Property::CidLink => {
            let name = format_ident!("todo4_{}", name);
            (name, quote! { String }, None)
        }
        Property::Bytes => {
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
    name: &String,
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
        let (property_code, additional_code) =
            gen_property(&name, property, is_required, is_nullable);

        properties_code.push(property_code);
        additional_declarations.push(additional_code);
    }

    let name = format_ident!("{}", name.to_case(Case::Pascal));

    quote! {
        #(#additional_declarations)*
        pub struct #name {
            #(#properties_code)*
        }
    }
}

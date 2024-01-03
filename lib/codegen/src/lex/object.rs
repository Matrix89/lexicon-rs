use convert_case::{Case, Casing};
use lexicon::lexicon::{Object, ObjectField};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::{doc_builder::DocBuilder, CodeGen};

use super::{array::gen_array, r#ref::gen_ref_variant};

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
        "ref" => format!("r#{}", name),
        "type" => format!("r#{}", name),
        v => String::from(v),
    };

    format_ident!("{}", name)
}

impl CodeGen {
    fn gen_property(
        &self,
        name: &str,
        namespace: &str,
        object_name: &str,
        property: ObjectField,
        is_required: bool,
        is_nullable: bool,
    ) -> (TokenStream, Option<TokenStream>) {
        let mut doc = DocBuilder::new();

        let (name, prop_type, additional_code) = match property {
            ObjectField::Primitive(primitive) => {
                self.gen_primitive(name, primitive, namespace, &mut doc)
            }
            ObjectField::RefVariant(variant) => {
                let (enum_name, r#enum) =
                    gen_ref_variant(&format!("{}{}", object_name, name), variant, namespace);
                let name = gen_field_name(name);
                (name, enum_name, r#enum)
            }
            ObjectField::Blob(ref blob) => {
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

                gen_array(name, array, namespace)
            }
            ObjectField::Unknown => (
                format_ident!("{}", name),
                quote! { ::serde_json::Value },
                None,
            ),
        };

        let prop = if is_nullable || !is_required {
            quote! {
                #[serde(skip_serializing_if = "Option::is_none")]
                pub #name: Option<#prop_type>,
            }
        } else {
            quote! { pub #name: #prop_type, }
        };

        doc.add_item("nullable", is_nullable);
        doc.add_item("required", is_required);

        let doc = if self.docs {
            doc.build()
        } else {
            quote! {}
        };

        let property_code = quote! {
            #doc
            #prop
        };

        (property_code, additional_code)
    }

    pub fn gen_object(&self, object_name: &str, namespace: &str, object: Object) -> TokenStream {
        let properties = object.properties.unwrap_or_default();
        let required = object.required.unwrap_or_default();
        let nullable = object.nullable.unwrap_or_default();

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
            #[serde(rename_all = "camelCase")]
            pub struct #name {
                #(#properties_code)*
            }
        }
    }
}

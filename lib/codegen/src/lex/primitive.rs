use lexicon::lexicon::Primitive;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::{doc_builder::DocBuilder, CodeGen};

use super::{object::gen_field_name, union::gen_union};

impl CodeGen {
    pub fn gen_primitive(
        &self,
        name: &String,
        primitive: Primitive,
        ns: &String,
        doc: &mut DocBuilder,
    ) -> (Ident, TokenStream, Option<TokenStream>) {
        match primitive {
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
                        let (union_name, code) = gen_union(name, know_values.clone(), ns);
                        let name = gen_field_name(name);
                        let prop_type = quote! { #union_name };
                        (name, prop_type, Some(code))
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
        }
    }
}

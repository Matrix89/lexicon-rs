use lexicon::lexicon::{LexString, Primitive};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::{doc_builder::DocBuilder, CodeGen};

use super::{object::gen_field_name, union::gen_union};

impl CodeGen {
    pub fn gen_primitive(
        &self,
        name: &str,
        primitive: Primitive,
        ns: &str,
        doc: &mut DocBuilder,
    ) -> (Ident, TokenStream, Option<TokenStream>) {
        match primitive {
            Primitive::String(str) => match str {
                LexString::Datetime(datetime) => {
                    doc.add_optional_item("description", &datetime.description);
                    let name = gen_field_name(name);
                    let prop_type = quote! { chrono::DateTime<chrono::Utc> };
                    (name, prop_type, None)
                }
                LexString::Did(did) => {
                    doc.add_optional_item("description", &did.description);
                    let name = gen_field_name(name);
                    let prop_type = quote! { lexicon::did::Did };
                    (name, prop_type, None)
                }
                LexString::Handle(handle) => {
                    doc.add_optional_item("description", &handle.description);
                    let name = gen_field_name(name);
                    let prop_type = quote! { lexicon::handle::Handle };
                    (name, prop_type, None)
                }
                LexString::Uri(uri) => {
                    doc.add_optional_item("description", &uri.description);
                    let name = gen_field_name(name);
                    let prop_type = quote! { lexicon::url::Url };
                    (name, prop_type, None)
                }
                LexString::Cid(cid) => {
                    doc.add_optional_item("description", &cid.description);
                    let name = gen_field_name(name);
                    let prop_type = quote! { lexicon::cid::Cid };
                    (name, prop_type, None)
                }
                LexString::AtUri(uri) => {
                    doc.add_optional_item("description", &uri.description);
                    let name = gen_field_name(name);
                    let prop_type = quote! { lexicon::at_uri::AtUri };
                    (name, prop_type, None)
                }
                LexString::AtIdentifier(identifier) => {
                    doc.add_optional_item("description", &identifier.description);
                    let name = gen_field_name(name);
                    let prop_type = quote! { lexicon::at_identifier::AtIdentifier };
                    (name, prop_type, None)
                }
                LexString::Nsid(nsid) => {
                    doc.add_optional_item("description", &nsid.description);
                    let name = gen_field_name(name);
                    let prop_type = quote! { lexicon::nsid::Nsid };
                    (name, prop_type, None)
                }
                LexString::OtherString(str) => {
                    doc.add_optional_item("description", &str.description);
                    doc.add_optional_item("default", &str.default);
                    doc.add_optional_item("max_length", &str.max_length);
                    doc.add_optional_item("max_graphemes", &str.max_graphemes);
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
                LexString::Language => {
                    let name = gen_field_name(name);
                    let prop_type = quote! { lexicon::language::Language };
                    (name, prop_type, None)
                }
            },
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

use lexicon::lexicon::{Array, ArrayItem, Primitive};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use super::{object::gen_field_name, r#ref::gen_ref_variant};

pub fn gen_array(name: &str, array: Array, ns: &str) -> (Ident, TokenStream, Option<TokenStream>) {
    let (array_type, additional_code) = match *array.items {
        ArrayItem::Primitive(Primitive::String(_)) => (quote! { String }, None),
        ArrayItem::Primitive(Primitive::Boolean(_)) => (quote! { bool }, None),
        ArrayItem::Primitive(Primitive::Integer(_)) => (quote! { i64 }, None),
        ArrayItem::RefVariant(variant) => gen_ref_variant(name, variant, ns),
        ArrayItem::Unknown(item) => {
            println!("unknown array item: {}", item);
            (quote! { Unimplemented }, None)
        }
    };
    let name = gen_field_name(name);

    let r#type = quote! { Vec<#array_type> };
    (name, r#type, additional_code)
}

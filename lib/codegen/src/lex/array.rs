use lexicon::lexicon::{Array, ArrayItem, Primitive, RefVariant};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use super::{
    object::{build_ref_target, gen_field_name},
    r#ref::gen_ref_variant,
    union::gen_union,
};

pub fn gen_array(name: &String, array: Array) -> (Ident, TokenStream, Option<TokenStream>) {
    let (array_type, additional_code) = match *array.items {
        ArrayItem::Primitive(Primitive::String(_)) => (quote! { String }, None),
        ArrayItem::Primitive(Primitive::Boolean(_)) => (quote! { bool }, None),
        ArrayItem::Primitive(Primitive::Integer(_)) => (quote! { i64 }, None),
        ArrayItem::RefVariant(variant) => gen_ref_variant(name, variant),
        ArrayItem::Ref(r#ref) => {
            let ref_target = build_ref_target(&r#ref.r#ref);
            (quote! { #ref_target }, None)
        }
        ArrayItem::Unknown(item) => {
            println!("unknown array item: {}", item);
            (quote! { String }, None)
        }
    };
    let name = gen_field_name(name);

    let r#type = quote! { Vec<#array_type> };
    (name, r#type, additional_code)
}

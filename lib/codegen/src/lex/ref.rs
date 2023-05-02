use lexicon::lexicon::RefVariant;
use proc_macro2::TokenStream;
use quote::quote;

use super::{object::build_ref_target, union::gen_union};

pub fn gen_ref_variant(name: &String, variant: RefVariant) -> (TokenStream, Option<TokenStream>) {
    match variant {
        RefVariant::Ref(r#ref) => {
            let ref_target = build_ref_target(&r#ref.r#ref);
            (quote! { #ref_target }, None)
        }
        RefVariant::Union(union) => {
            let (enum_name, r#enum) = gen_union(name, union.refs);
            (quote! { #enum_name }, Some(r#enum))
        }
    }
}

use proc_macro2::TokenStream;

mod doc_builder;
pub mod lex;
pub mod xrpc;

pub fn gen() -> TokenStream {
    quote::quote! {}
}

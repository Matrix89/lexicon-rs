use proc_macro2::TokenStream;
use quote::quote;

pub fn gen_preamble() -> TokenStream {
    quote! {
        struct Xrpc {
        }
    }
}

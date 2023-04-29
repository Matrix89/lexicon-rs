use convert_case::{Case, Casing};
use lexicon::lexicon::{Parameters, JV};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::xrpc::parameters::gen_parameters;

pub fn gen_procedure(
    name: String,
    description: Option<String>,
    parameters: Option<Parameters>,
    input: Option</* TODO */ JV>,
    output: Option</* TODO */ JV>,
    errors: Option<Vec</* TODO */ JV>>,
) -> TokenStream {
    let name = format_ident!("{}", name.to_case(Case::Snake));
    //let desc = format!("{}", description.unwrap_or("no desc".to_owned()));
    let parameters = if let Some(parameters) = parameters {
        gen_parameters(parameters)
    } else {
        quote! {}
    };
    quote! {
        //#[doc=#desc]
        pub fn #name(#parameters) {}
    }
}

use convert_case::{Case, Casing};
use lexicon::lexicon::{Parameters, XrpcSubscription, JV};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::CodeGen;

fn gen_body(namespace: &String, parameters: &Parameters) -> TokenStream {
    let url = format!(
        "wss://bsky.social/xrpc/{}",
        namespace.replace("::", ".").replace(".lexicon.", ""),
    );
    let params = parameters
        .clone()
        .properties
        .unwrap_or_default()
        .iter()
        .filter(|(k, v)| matches!(v, lexicon::lexicon::Parameter::Primitive(_)))
        .map(|(k, v)| {
            let ident = format_ident!("{}", k.to_case(Case::Snake));
            quote! {
                .param(#k.to_string(), #ident.to_string())
            }
        })
        .collect::<TokenStream>();
    quote! {
        let query = XrpcSubscription::new(#url.to_string())
            #params
        .token(token);
        query.subscribe();
    }
}
impl CodeGen {
    pub fn gen_subscription(
        &self,
        namespace: &String,
        name: &String,
        sub: XrpcSubscription,
    ) -> TokenStream {
        let name = format_ident!("{}", name.to_case(Case::Snake));
        let desc = format!("{}", sub.description.unwrap_or("no desc".to_owned()));
        let params = sub.parameters.unwrap_or_default();
        let body = gen_body(&namespace, &params);
        let parameters = self.gen_parameters(&params);
        quote! {
            #[doc=#desc]
            use xrpc::error::XrpcError;
            use xrpc::subscription::XrpcSubscription;
            pub fn #name(token: &String, #parameters) {
                #body
            }
        }
    }
}

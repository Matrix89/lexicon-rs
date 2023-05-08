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
    let params = CodeGen::gen_parameters_body(&parameters.properties);
    quote! {
        let mut req = XrpcSubscription::new(#url.to_string());
        #params
        req.token(token);
        req.subscribe();
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
        let arguments = self.gen_arguments(&params);
        quote! {
            #[doc=#desc]
            use xrpc::error::XrpcError;
            use xrpc::subscription::XrpcSubscription;
            pub fn #name(token: &String, #arguments) {
                #body
            }
        }
    }
}

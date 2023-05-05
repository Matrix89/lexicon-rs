use convert_case::{Case, Casing};
use lexicon::lexicon::{Parameters, XrpcQuery};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{doc_builder::DocBuilder, CodeGen};

fn gen_body(
    name: &str,
    namespace: &String,
    parameters: Parameters,
    output_type: &TokenStream,
    output: &TokenStream,
) -> TokenStream {
    let url = format!(
        "https://bsky.social/xrpc/{}",
        namespace.replace("::", ".").replace(".lexicon.", ""),
    );
    let params = parameters
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
        /*let client = reqwest::blocking::Client::new();
        return client
            .get(#url)
            .header("Authorization", token)
            .send()?
            .json::<#output_type>();*/
        let query = XrpcQuery::new(#url.to_string())
            #params
        .token(token);
        query.execute::<#output_type>()
    }
}

impl CodeGen {
    pub fn gen_query(&self, namespace: &String, name: &String, query: XrpcQuery) -> TokenStream {
        let mut doc = DocBuilder::new();
        doc.add_optional_item("Description", &query.description);

        let queryParams = query.parameters.unwrap_or_default();
        let parameters = self.gen_parameters(&queryParams);
        let (output_type, output) = self.gen_body(
            &format!("{}Output", name.to_case(Case::Pascal)),
            query.output.unwrap_or_default(),
        );
        let body = gen_body(name, namespace, queryParams, &output_type, &output);

        let name = format_ident!("{}", name.to_case(Case::Snake));

        let doc = doc.build();

        quote! {
            #output
            #doc
            use xrpc::error::XrpcError;
            use xrpc::query::XrpcQuery;
            pub fn #name(token: &String, #parameters) -> Result<#output_type, XrpcError> {
                #body
            }
        }
    }
}

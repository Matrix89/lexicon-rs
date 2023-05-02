use convert_case::{Case, Casing};
use lexicon::lexicon::{Parameters, UserType, XrpcBody, XrpcQuery, JV};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    doc_builder::DocBuilder, lex::object::build_ref_target, xrpc::parameters::gen_parameters,
    CodeGen,
};

fn gen_body(
    name: &str,
    namespace: &String,
    output_type: &TokenStream,
    output: &TokenStream,
) -> TokenStream {
    let url = format!(
        "http://bsky.social/xrpc/{}",
        namespace.replace("::", ".").replace(".lexicon.", ""),
    );
    quote! {
        let client = reqwest::blocking::Client::new();
        return client.get(#url).header("Authorization", token).send()?.json::<#output_type>();
    }
}

impl CodeGen {
    pub fn gen_query(&self, namespace: &String, name: &String, query: XrpcQuery) -> TokenStream {
        let mut doc = DocBuilder::new();
        doc.add_optional_item("Description", &query.description);

        let parameters = gen_parameters(query.parameters.unwrap_or_default());
        let (output_type, output) = self.gen_body(
            &format!("{}Output", name.to_case(Case::Pascal)),
            query.output.unwrap_or_default(),
        );
        let body = gen_body(name, namespace, &output_type, &output);

        let name = format_ident!("{}", name.to_case(Case::Snake));

        let doc = doc.build();

        quote! {
            #output
            #doc
            pub fn #name(token: &String, #parameters) -> Result<#output_type, reqwest::Error> {
                #body
            }
        }
    }
}

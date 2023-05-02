use convert_case::{Case, Casing};
use lexicon::lexicon::{Parameters, XrpcBody, XrpcProcedure, JV};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{doc_builder::DocBuilder, xrpc::parameters::gen_parameters, CodeGen};

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
        return client.post(#url).header("Authorization", token).send()?.json::<#output_type>();
    }
}

impl CodeGen {
    pub fn gen_procedure(
        &self,
        namespace: &String,
        name: &String,
        proc: XrpcProcedure,
    ) -> TokenStream {
        println!("Generating query: {} {}", namespace, name);
        let mut doc = DocBuilder::new();
        doc.add_optional_item("Description", &proc.description);

        let parameters = gen_parameters(proc.parameters.unwrap_or_default());
        let (output_type, output) = self.gen_body(&name, proc.output.unwrap_or_default());
        let body = gen_body(&name, &namespace, &output_type, &output);

        let name = format_ident!("{}", name.to_case(Case::Snake));

        let doc = doc.build();

        quote! {
            #output
            #doc
            pub fn #name(token: &String, #parameters) -> Option<#output_type> {
                #body
            }
        }
    }
}

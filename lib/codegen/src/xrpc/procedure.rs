use convert_case::{Case, Casing};
use lexicon::lexicon::{Output, Parameters, JV};
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
        "http://localhost:8080/xrpc{}",
        namespace.replace("::", "/").replace("/lexicon", ""),
    );
    quote! {
        let client = reqwest::blocking::Client::new();
        client.post(#url);

        return None;
    }
}

impl CodeGen {
    pub fn gen_procedure(
        &self,
        namespace: &String,
        name: &String,
        description: Option<String>,
        parameters: Option<Parameters>,
        input: Option<JV>,
        output: Option<Output>,
        errors: Option<Vec</* TODO */ JV>>,
    ) -> TokenStream {
        println!("Generating query: {} {}", namespace, name);
        let mut doc = DocBuilder::new();
        doc.add_optional_item("Description", description);

        let parameters = gen_parameters(parameters.unwrap_or_default());
        let (output_type, output) = self.gen_output(&name, output.unwrap_or_default());
        let body = gen_body(&name, &namespace, &output_type, &output);

        let name = format_ident!("{}", name.to_case(Case::Snake));

        let doc = doc.build();

        quote! {
            #output
            #doc
            pub fn #name(#parameters) -> Option<#output_type> {
                #body
            }
        }
    }
}

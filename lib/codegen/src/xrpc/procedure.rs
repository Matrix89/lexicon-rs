use convert_case::{Case, Casing};
use lexicon::lexicon::XrpcProcedure;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{doc_builder::DocBuilder, CodeGen};

fn gen_body(namespace: &str, output_type: &TokenStream) -> TokenStream {
    let url = format!(
        "https://bsky.social/xrpc/{}",
        namespace.replace("::", ".").replace(".lexicon.", ""),
    );

    let input = format_ident!("input");

    quote! {
        let proc = XrpcProcedure::new(#url.to_string())
            .input(#input)
            .token(token);
        proc.execute::<#output_type>().await
    }
}

impl CodeGen {
    pub fn gen_procedure(&self, namespace: &str, name: &str, proc: XrpcProcedure) -> TokenStream {
        let mut doc = DocBuilder::new();
        doc.add_optional_item("Description", &proc.description);

        let arguments = self.gen_arguments(&proc.parameters.unwrap_or_default());
        let (output_type, output) = self.gen_body(
            &format!("{}Output", name.to_case(Case::Pascal)),
            proc.output.unwrap_or_default(),
        );

        let (input_type, input) = self.gen_body(
            &format!("{}Input", name.to_case(Case::Pascal)),
            proc.input.unwrap_or_default(),
        );

        let body = gen_body(namespace, &output_type);

        let name = format_ident!("{}", name.to_case(Case::Snake));

        let doc = doc.build();

        quote! {
            use xrpc::error::XrpcError;
            use xrpc::procedure::XrpcProcedure;
            #output
            #input
            #doc
            pub async fn #name(token: &String, input: #input_type, #arguments) -> Result<#output_type, XrpcError> {
                #body
            }
        }
    }
}

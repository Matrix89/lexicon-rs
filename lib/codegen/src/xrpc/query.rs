use convert_case::{Case, Casing};
use lexicon::lexicon::{Output, Parameters, JV};
use proc_macro2::{Ident, TokenStream};
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
        "http://bsky.social/xrpc{}",
        namespace.replace("::", ".").replace("/lexicon", ""),
    );
    quote! {
        let client = reqwest::blocking::Client::new();
        client.get(#url);

        return None;
    }
}

impl CodeGen {
    pub fn gen_output(&self, name: &str, output: Output) -> (TokenStream, TokenStream) {
        match output.encoding {
            Some(encoding) if encoding.as_str() == "application/json" => {
                let output_name = format!("{}Output", name.to_case(Case::Pascal));
                let schema = *output.schema.unwrap();

                let code = match schema {
                    lexicon::lexicon::UserType::Object {
                        description,
                        required,
                        nullable,
                        properties,
                    } => self.gen_object(
                        &output_name,
                        &"".to_owned(),
                        description.unwrap_or_else(|| "".to_owned()),
                        required.unwrap_or_default(),
                        nullable.unwrap_or_default(),
                        properties.unwrap_or_default(),
                    ),
                    lexicon::lexicon::UserType::Ref { description, r#ref } => {
                        let target = build_ref_target(&r#ref);
                        return (quote! { #target }, quote! {});
                    }
                    _ => {
                        println!("Unsupported output type: {:?}", schema);
                        quote! {}
                    }
                };

                let t = format_ident!("{}", output_name);
                (quote! {#t}, code)
            }
            Some(v) => {
                println!("Unsupported output encoding: {:?}", v);
                (quote! {()}, quote! {})
            }
            _ => {
                println!("Missing output type: {:?}", output);
                (quote! {()}, quote! {})
            }
        }
    }

    pub fn gen_query(
        &self,
        namespace: &String,
        name: &String,
        description: Option<String>,
        parameters: Option<Parameters>,
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

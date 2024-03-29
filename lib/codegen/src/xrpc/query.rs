use convert_case::{Case, Casing};
use lexicon::lexicon::{LexString, Parameter, Parameters, Primitive, XrpcQuery};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{doc_builder::DocBuilder, CodeGen};

fn gen_body(namespace: &str, parameters: &Parameters, output_type: &TokenStream) -> TokenStream {
    let url = format!(
        "https://bsky.social/xrpc/{}",
        namespace.replace("::", ".").replace(".lexicon.", ""),
    );
    let params = CodeGen::gen_parameters_body(&parameters.properties);
    quote! {
        let mut req = XrpcQuery::new(#url.to_string());
        #params
        req.token(token);
        req.execute::<#output_type>().await
    }
}

impl CodeGen {
    pub fn gen_query(&self, namespace: &str, name: &str, query: XrpcQuery) -> TokenStream {
        let arguments = self.gen_arguments(&query.parameters);
        let (output_type, output) = self.gen_output(name, query.output.unwrap_or_default());
        let body = gen_body(namespace, &query.parameters, &output_type);

        let name = format_ident!("{}", name.to_case(Case::Snake));

        let doc = {
            let mut doc = DocBuilder::new();
            doc.add_optional_item("Description", &query.description);
            for (name, prop) in query.parameters.properties {
                let desc = match prop {
                    Parameter::Array(arr) => arr.description.clone(),
                    Parameter::Primitive(primitive) => match primitive {
                        Primitive::String(format) => match format {
                            LexString::AtIdentifier(identifier) => identifier.description.clone(),
                            LexString::OtherString(string) => string.description.clone(),
                            LexString::Uri(uri) => uri.description.clone(),
                            LexString::AtUri(uri) => uri.description.clone(),
                            LexString::Cid(cid) => cid.description.clone(),
                            LexString::Datetime(datetime) => datetime.description.clone(),
                            LexString::Did(did) => did.description.clone(),
                            LexString::Handle(handle) => handle.description.clone(),
                            LexString::Nsid(nsid) => nsid.description.clone(),
                            v => todo!("format: {:?}", v),
                        },
                        Primitive::Boolean(bool) => bool.description.clone(),
                        Primitive::Integer(int) => int.description.clone(),
                    },
                };
                doc.add_argument(&name, desc);
            }
            doc.build()
        };

        quote! {
            use xrpc::error::XrpcError;
            use xrpc::query::XrpcQuery;
            #output
            #doc
            pub async fn #name(token: &String, #arguments) -> Result<#output_type, XrpcError> {
                #body
            }
        }
    }
}

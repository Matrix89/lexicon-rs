use convert_case::{Case, Casing};
use lexicon::lexicon::{XrpcBody, XrpcBodySchema};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{lex::r#ref::gen_ref_variant, CodeGen};

impl CodeGen {
    pub fn gen_output(&self, name: &String, body: XrpcBody) -> (TokenStream, TokenStream) {
        let name = format!("{}Output", name.to_case(Case::Pascal));
        self.gen_body(&name, body)
    }

    pub fn gen_body(&self, name: &String, body: XrpcBody) -> (TokenStream, TokenStream) {
        match &body.encoding {
            Some(encoding) if encoding.as_str() == "application/json" => {
                let name = format!("{}", name.to_case(Case::Pascal));
                let schema = body.schema.unwrap();

                let code = match schema {
                    XrpcBodySchema::Object(obj) => self.gen_object(&name, &"".to_owned(), obj),
                    XrpcBodySchema::RefVariant(variant) => {
                        return (gen_ref_variant(&name, variant, &"".to_owned()).0, quote! {})
                    }
                    t => todo!("{:?}", t),
                };

                let name = format_ident!("{}", name);
                (quote! {#name}, code)
            }
            Some(v) => {
                println!("Unsupported body encoding: {:?}", v);
                (quote! {()}, quote! {})
            }
            _ => (quote! {()}, quote! {}),
        }
    }
}

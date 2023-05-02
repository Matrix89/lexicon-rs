use convert_case::{Case, Casing};
use lexicon::lexicon::{XrpcBody, XrpcBodySchema};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{lex::r#ref::gen_ref_variant, CodeGen};

impl CodeGen {
    pub fn gen_body(&self, name: &String, body: XrpcBody) -> (TokenStream, TokenStream) {
        match body.encoding {
            Some(encoding) if encoding.as_str() == "application/json" => {
                let output_name = format!("{}", name.to_case(Case::Pascal));
                let schema = body.schema.unwrap();

                let code = match schema {
                    XrpcBodySchema::Object(obj) => self.gen_object(
                        &output_name,
                        &"".to_owned(),
                        obj.description.unwrap_or_else(|| "".to_owned()),
                        obj.required.unwrap_or_default(),
                        obj.nullable.unwrap_or_default(),
                        obj.properties.unwrap_or_default(),
                    ),
                    XrpcBodySchema::RefVariant(variant) => {
                        return (gen_ref_variant(name, variant).0, quote! {})
                    }

                    t => todo!("{:?}", t),
                };

                let t = format_ident!("{}", name);
                (quote! {#t}, code)
            }
            Some(v) => {
                println!("Unsupported output encoding: {:?}", v);
                (quote! {()}, quote! {})
            }
            _ => {
                println!("Missing output type: {:?}", body);
                (quote! {()}, quote! {})
            }
        }
    }
}

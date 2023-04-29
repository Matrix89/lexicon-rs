use convert_case::{Case, Casing};
use lexicon::lexicon::{Output, Parameters, JV};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::{lex::object::gen_object, xrpc::parameters::gen_parameters};

fn gen_output(name: String, output: Output) -> (Ident, TokenStream) {
    if output.encoding.as_ref().unwrap() != "application/json" {
        println!(
            "Unsupported output type: {}",
            output.encoding.as_ref().unwrap()
        );

        (format_ident!("todo"), quote! {})
    } else {
        let output_name = format!("{}Output", name.to_case(Case::Pascal));
        let schema = *output.schema.unwrap();
        let code = match schema {
            lexicon::lexicon::UserType::Object {
                description,
                required,
                nullable,
                properties,
            } => gen_object(
                &output_name,
                description.unwrap_or_else(|| "".to_owned()),
                required.unwrap_or_default(),
                nullable.unwrap_or_default(),
                properties.unwrap_or_default(),
            ),
            _ => {
                println!("Unsupported output type: {:?}", schema);
                quote! {}
            }
        };

        (format_ident!("{}", output_name), code)
    }
}

pub fn gen_query(
    name: String,
    description: Option<String>,
    parameters: Option<Parameters>,
    output: Option<Output>,
    errors: Option<Vec</* TODO */ JV>>,
) -> TokenStream {
    let desc = format!("{}", description.unwrap_or("no desc".to_owned()));
    let parameters = if let Some(parameters) = parameters {
        gen_parameters(parameters)
    } else {
        quote! {}
    };
    let (output_type, output) = if output.is_none() {
        (format_ident!("todo"), quote! {})
    } else {
        gen_output(name.clone(), output.unwrap())
    };

    let name = format_ident!("{}", name.to_case(Case::Snake));
    quote! {
        #output
        #[doc=#desc]
        pub fn #name(#parameters) -> #output_type {}
    }
}

use convert_case::{Case, Casing};
use lexicon::lexicon::{LexiconDoc, Parameters, Property, UserType};
use nsid::NSIDNode;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rust_format::{Formatter, RustFmt};
use std::{collections::HashMap, fs, str::FromStr};
use walkdir::WalkDir;

fn main() {
    let lexicons = WalkDir::new("/home/matrix89/dev/atproto/lexicons")
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|e| e.file_type().is_file())
        .map(|file| (file.path().to_owned(), fs::read_to_string(file.path())))
        .filter(|f| f.1.is_ok())
        .map(|(path, contents)| (path, contents.unwrap()))
        .map(|(path, contents)| {
            LexiconDoc::from_str(contents.as_str())
                .unwrap_or_else(|err| panic!("Failed to load lexicon: {}, {}", path.display(), err))
        });

    let mut root = NSIDNode::root();

    for lexicon in lexicons {
        //println!("{}:", lexicon.id);
        root.add(&lexicon.id, lexicon.defs);
    }

    let tokens = gen(root);
    println!("Gen ok");
    let src = RustFmt::default()
        .format_str(tokens.to_string().replace("{", "{\n").replace("}", "}\n"))
        .unwrap();
    println!("{}", src);
}

fn gen_def(def: (String, UserType)) -> TokenStream {
    let name = format_ident!("{}", def.0.to_case(Case::Pascal));
    match def.1 {
        UserType::Object {
            description,
            required,
            nullable,
            properties,
        } => codegen::lex::object::gen_object(
            &def.0,
            description.unwrap_or_else(|| "".to_owned()),
            required.unwrap_or_default(),
            nullable.unwrap_or_default(),
            properties.unwrap_or_default(),
        ),
        UserType::XrpcQuery {
            description,
            parameters,
            output,
            errors,
        } => codegen::xrpc::query::gen_query(def.0, description, parameters, output, errors),
        UserType::XrpcProcedure {
            description,
            parameters,
            input,
            output,
            errors,
        } => codegen::xrpc::procedure::gen_procedure(
            def.0,
            description,
            parameters,
            input,
            output,
            errors,
        ),
        UserType::XrpcSubscription {
            description,
            parameters,
            message,
            infos,
            errors,
        } => codegen::xrpc::subscription::gen_subscription(
            def.0,
            description,
            parameters,
            message,
            infos,
            errors,
        ),
        UserType::Token { description } => {
            let name = format_ident!("{}", def.0);
            let desc = format!("{:?}", description);
            quote! {
                #[doc=#desc]
                pub struct #name {}
            }
        }
        v => {
            let name = format_ident!("todo_{}", def.0);
            let desc = format!("{:?}", v);
            quote! {
                #[doc=#desc]
                pub struct #name {}
            }
        }
    }
}

fn gen(tree: NSIDNode) -> TokenStream {
    match tree {
        NSIDNode::Segment { name, children } => {
            let children = children.into_iter().map(gen);
            let name = format_ident!("{}", name.to_case(Case::UpperCamel));
            quote! {
                pub mod #name {
                    #(#children)*
                }
            }
        }
        NSIDNode::Identifier { name, def } => {
            let defs = def.into_iter().map(gen_def);
            let name = format_ident!("{}", name.to_case(Case::UpperCamel));
            quote! {
                pub mod #name {
                    #(#defs)*
                }
            }
        }
    }
}

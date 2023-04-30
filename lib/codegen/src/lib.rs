use convert_case::{Case, Casing};
use lex::string::gen_string;
use lexicon::lexicon::UserType;
use nsid::NSIDNode;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

mod doc_builder;
pub mod lex;
pub mod xrpc;

pub struct CodeGen {
    tree: NSIDNode,
}

impl CodeGen {
    pub fn new(tree: NSIDNode) -> Self {
        Self { tree }
    }

    fn gen_def(self: &CodeGen, namespace: &String, def: (String, UserType)) -> TokenStream {
        let name = format_ident!("{}", def.0.to_case(Case::Pascal));
        match def.1 {
            UserType::Object {
                description,
                required,
                nullable,
                properties,
            } => self.gen_object(
                &def.0,
                namespace,
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
            } => self.gen_query(namespace, &def.0, description, parameters, output, errors),
            UserType::XrpcProcedure {
                description,
                parameters,
                input,
                output,
                errors,
            } => self.gen_procedure(
                namespace,
                &def.0,
                description,
                parameters,
                input,
                output,
                errors,
            ),
            /*UserType::XrpcSubscription {
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
            }*/
            UserType::XrpcProcedure { .. } => quote! {},
            UserType::XrpcSubscription { .. } => quote! {},
            UserType::String {
                description,
                default,
                min_length,
                max_length,
                min_graphemes,
                max_graphemes,
                r#enum,
                r#const,
                known_values,
            } => gen_string(
                &def.0,
                &self.tree,
                namespace,
                None,
                description,
                default,
                min_length,
                max_length,
                min_graphemes,
                max_graphemes,
                r#enum,
                r#const,
                known_values,
            ),
            UserType::Token { .. } => quote! {},
            _ => {
                println!("TODO! {:?}", def);
                quote! {}
            }
        }
    }

    pub fn gen(self: &CodeGen, node: NSIDNode, namespace: &String) -> TokenStream {
        match node {
            NSIDNode::Segment { name, children } => {
                let children = children.into_iter().map(|child| {
                    self.gen(
                        child,
                        &format!("{}::{}", namespace, name)
                            .to_owned()
                            .replace("/main", ""),
                    )
                });
                let name = format_ident!("{}", name.to_case(Case::UpperCamel));

                quote! {
                    pub mod #name {
                        #(#children)*
                    }
                }
            }
            NSIDNode::Identifier { name, def } => {
                let defs = def.into_iter().map(|def| {
                    self.gen_def(
                        &format!("{}::{}", namespace, name)
                            .to_owned()
                            .replace("/main", ""),
                        def,
                    )
                });
                let name = format_ident!("{}", name.to_case(Case::UpperCamel));
                quote! {
                    pub mod #name {
                        #(#defs)*
                    }
                }
            }
        }
    }
}

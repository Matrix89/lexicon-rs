use convert_case::{Case, Casing};
use lex::union::gen_union;
use lexicon::lexicon::UserType;
use nsid::NSIDNode;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

mod doc_builder;
pub mod lex;
pub mod xrpc;

pub struct CodeGen {
    tree: NSIDNode,

    docs: bool,
}

impl CodeGen {
    pub fn new(tree: NSIDNode) -> Self {
        Self { tree, docs: false }
    }

    fn gen_def(self: &CodeGen, namespace: &String, def: (String, UserType)) -> TokenStream {
        match def.1 {
            UserType::Object(obj) => self.gen_object(&def.0, namespace, obj),
            UserType::XrpcQuery(query) => self.gen_query(namespace, &def.0, query),
            UserType::XrpcProcedure(proc) => self.gen_procedure(namespace, &def.0, proc),
            UserType::String(str) => {
                gen_union(&def.0, str.known_values.unwrap_or_default(), namespace).1
            }
            UserType::Token(token) => {
                let name = format_ident!("{}", def.0.to_case(Case::Pascal));
                quote! {
                    #[derive(Debug, Clone, PartialEq, Eq, ::serde::Deserialize, ::serde::Serialize)]
                    pub struct #name;
                }
            }
            _ => {
                println!("Unknown top level UserType: {:?}", def);
                quote! {}
            }
        }
    }

    pub fn gen_lexicon(self: &CodeGen, node: NSIDNode, namespace: &String) -> TokenStream {
        match node {
            NSIDNode::Segment { name, children } => {
                let children = children.into_iter().map(|child| {
                    self.gen_lexicon(
                        child,
                        &format!("{}::{}", namespace, name)
                            .to_owned()
                            .replace("/main", ""),
                    )
                });
                let name = format_ident!("{}", name.to_case(Case::Snake));

                quote! {
                    pub mod #name {
                        #[allow(unused_imports)]
                        use super::lexicon;
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
                let name = format_ident!("{}", name.to_case(Case::Snake));
                quote! {
                    pub mod #name {
                        #[allow(unused_imports)]
                        use super::lexicon;
                        #[derive(Debug, Clone, PartialEq, Eq, ::serde::Serialize, ::serde::Deserialize)]
                        pub struct Unimplemented;
                        #(#defs)*
                    }
                }
            }
        }
    }

    pub fn gen(self: &CodeGen, node: NSIDNode, namespace: &String) -> TokenStream {
        let lexicon = self.gen_lexicon(node, namespace);

        let xrpc_preamble = xrpc::preamble::gen_preamble();
        let lexicon_preamble = quote! {};

        quote! {
            #lexicon_preamble
            #xrpc_preamble
            #lexicon
        }
    }
}

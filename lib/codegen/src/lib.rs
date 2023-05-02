use convert_case::{Case, Casing};
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
            UserType::Object(obj) => self.gen_object(
                &def.0,
                namespace,
                obj.description.unwrap_or_else(|| "".to_owned()),
                obj.required.unwrap_or_default(),
                obj.nullable.unwrap_or_default(),
                obj.properties.unwrap_or_default(),
            ),
            UserType::XrpcQuery(query) => self.gen_query(namespace, &def.0, query),
            UserType::XrpcProcedure(proc) => self.gen_procedure(namespace, &def.0, proc),
            UserType::XrpcSubscription { .. } => quote! {},
            UserType::Token { .. } => quote! {},
            _ => {
                println!("TODO! {:?}", def);
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

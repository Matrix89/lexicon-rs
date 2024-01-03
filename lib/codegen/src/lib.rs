use anyhow::{Context, Result};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use convert_case::{Case, Casing};
use lex::union::gen_union;
use lexicon::lexicon::{LexString, UserType};
use nsid::NSIDNode;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::lex::array::gen_array;

mod doc_builder;
pub mod lex;
pub mod xrpc;

pub struct CodeGen {
    docs: bool,
    base_path: PathBuf,
}

impl CodeGen {
    pub fn new(base_path: PathBuf) -> Self {
        Self {
            docs: true,
            base_path,
        }
    }

    fn gen_def(self: &CodeGen, namespace: &str, def: (String, UserType)) -> TokenStream {
        match def.1 {
            UserType::Object(obj) => self.gen_object(&def.0, namespace, obj),
            UserType::XrpcQuery(query) => self.gen_query(namespace, &def.0, query),
            UserType::XrpcProcedure(proc) => self.gen_procedure(namespace, &def.0, proc),
            UserType::XrpcSubscription(sub) => self.gen_subscription(namespace, &def.0, sub),
            UserType::String(str) => match str {
                LexString::OtherString(str) => {
                    if let Some(known_values) = str.known_values {
                        gen_union(&def.0, known_values, namespace).1
                    } else {
                        quote! {}
                    }
                }
                v => todo!("String {:?}", v),
            },
            UserType::Token(token) => {
                let desc = token.description.unwrap_or_default();
                let name = format_ident!("{}", def.0.to_case(Case::Pascal));
                quote! {
                    #[derive(Debug, Clone, PartialEq, Eq, ::serde::Deserialize, ::serde::Serialize)]
                    #[doc=#desc]
                    pub struct #name;
                }
            }
            UserType::Record(record) => self.gen_record(&def.0, record, namespace),
            UserType::Array(of) => {
                let (_, inner, additional_code) = gen_array(&def.0, of, namespace);
                let name = format_ident!("{}", def.0.to_case(Case::Pascal));
                quote! {
                    pub type #name = #inner;
                    #additional_code
                }
            }
            _ => {
                println!("Unsupported top level UserType: {:?}", def);
                quote! {}
            }
        }
    }

    pub fn gen_lexicon(
        self: &CodeGen,
        base_path: &Path,
        node: NSIDNode,
        namespace: &String,
        is_root: bool,
    ) -> Result<()> {
        match node {
            NSIDNode::Segment { name, children } => {
                let path = base_path.join(&name);
                fs::create_dir(&path)?;
                let children_names = children
                    .iter()
                    .map(|c| c.name())
                    .map(|name| format_ident!("{}", name.to_case(Case::Snake)))
                    .collect::<Vec<_>>();
                for child in children {
                    let name = format_ident!("{}", child.name().to_case(Case::Snake));

                    self.gen_lexicon(
                        &path,
                        child,
                        &format!("{}::{}", namespace, name)
                            .to_owned()
                            .replace("/main", ""),
                        false,
                    )?;
                }
                let mod_def = if is_root {
                    let xrpc_preamble = xrpc::preamble::gen_preamble();
                    let lexicon_preamble = quote! {
                        pub mod did {
                            pub type Did = String;
                        }
                        pub mod cid {
                            pub type Cid = String;
                        }
                        pub mod handle {
                            pub type Handle = String;
                        }
                        pub mod at_uri {
                            pub type AtUri = String;
                        }
                        pub mod at_identifier {
                            pub type AtIdentifier = String;
                        }
                        pub mod nsid {
                            pub type Nsid = String;
                        }
                        pub mod url {
                            pub type Url = String;
                        }
                    };

                    quote! {
                        pub mod lexicon {
                            #xrpc_preamble
                            #lexicon_preamble
                            #(pub use super::#children_names;)*
                        }
                        #(pub mod #children_names;)*
                    }
                } else {
                    quote! {
                        use super::lexicon;
                        #(pub mod #children_names;)*
                    }
                };
                write_source(path.join("mod.rs"), mod_def)
                    .with_context(|| format!("writing mod def {:?}", name))?;
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
                let source = quote! {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #(#defs)*
                };
                let file = base_path.join(format!("{}.rs", name));
                write_source(&file, source)
                    .with_context(|| format!("writing source for {}", file.display()))?;
            }
        };
        Ok(())
    }

    pub fn gen(self: &CodeGen, node: NSIDNode, namespace: &String) -> Result<()> {
        self.gen_lexicon(&self.base_path, node, namespace, true)?;

        Ok(())
    }
}

pub fn write_source<P: AsRef<Path>>(path: P, content: TokenStream) -> Result<()> {
    let mut formatted = Vec::new();
    let syntax_tree: syn::File = syn::parse2(content)?;
    let pretty = prettyplease::unparse(&syntax_tree);
    write!(formatted, "{}", pretty)?;
    fs::write(path, formatted)?;
    Ok(())
}

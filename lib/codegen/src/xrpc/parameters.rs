use convert_case::{Case, Casing};
use lexicon::lexicon::Parameters;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn gen_parameters(parameters: Parameters) -> TokenStream {
    let properties = parameters.properties.unwrap_or_default();
    let required = parameters.required.unwrap_or_default();

    let mut properties = properties.into_iter().collect::<Vec<_>>();
    properties.sort_by(|a, b| a.0.cmp(&b.0));

    properties
        .into_iter()
        .map(|(name, value)| {
            let name = format_ident!("{}", name.to_case(Case::Snake));
            /*match value {
                Primitive::String {
                    description,
                    format,
                    ..
                } => quote! { #name: String, },
                Primitive::Integer { .. } => quote! { #name: i64, },
                v => todo!("{:?}", v),
            }*/
            quote! {}
        })
        .collect()
}

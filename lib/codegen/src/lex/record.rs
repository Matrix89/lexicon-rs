use lexicon::lexicon::Record;
use proc_macro2::TokenStream;
use quote::quote;

use crate::CodeGen;

impl CodeGen {
    pub fn gen_record(&self, name: &String, record: Record, ns: &String) -> TokenStream {
        let object = self.gen_object(name, ns, record.record);
        println!("Finish up record generation");
        quote! {
            #object
        }
    }
}

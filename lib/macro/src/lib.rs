use codegen::CodeGen;
use nsid::NSIDNode;
use proc_macro::TokenStream;
use std::{fs, str::FromStr};
use walkdir::WalkDir;

/*#[proc_macro]
pub fn lexicon(_items: TokenStream) -> TokenStream {
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

    for lexicon in lexicons
    //.skip(50)
    /*.take(15)*/
    {
        //println!("{}:", lexicon.id);
        root.add(&lexicon.id, lexicon.defs);
    }

    let gen = CodeGen::new(root.clone());

    gen.gen(root, &"".to_owned()).into()
}*/

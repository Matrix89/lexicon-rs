use anyhow::Result;
use codegen::CodeGen;
use lexicon::doc::LexiconDoc;
use nsid::NSIDNode;
use std::{fs, str::FromStr};
use walkdir::WalkDir;

fn main() -> Result<()> {
    let lexicons = WalkDir::new("/home/matrix89/Development/atproto/lexicons")
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
        root.add(&lexicon.id, lexicon.defs);
    }

    let _ = fs::remove_dir_all("lib/test/src/lexicon");
    let gen = CodeGen::new("lib/test/src".into());

    gen.gen(root, &"".to_owned())?;
    // println!("Gen ok");
    // let src = RustFmt::default().format_str(tokens.to_string());
    // match src {
    //     Ok(src) => {
    //         fs::remove_file("lib/test/src/lex.rs").unwrap();
    //         fs::write("lib/test/src/lex.rs", src).unwrap();
    //     }
    //     Err(err) => println!("{}", err),
    // }
    Ok(())
}

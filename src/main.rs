use codegen::CodeGen;
use lexicon::lexicon::LexiconDoc;
use nsid::NSIDNode;
use rust_format::{Formatter, RustFmt};
use std::{fs, str::FromStr};
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

    for lexicon in lexicons
    //.skip(50)
    /*.take(15)*/
    {
        //println!("{}:", lexicon.id);
        root.add(&lexicon.id, lexicon.defs);
    }

    let gen = CodeGen::new(root.clone());

    let tokens = gen.gen(root, &"".to_owned());
    println!("Gen ok");
    let src = RustFmt::default().format_str(tokens.to_string()).unwrap();
    //println!("{}", src);

    fs::remove_file("/tmp/lex.rs").unwrap();
    fs::write("/tmp/lex.rs", src).unwrap();
    std::process::Command::new("rustc")
        .stdout(std::process::Stdio::inherit())
        .arg("--crate-type")
        .arg("lib")
        .arg("/tmp/lex.rs")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

mod lex;

fn main() {
    let res = lex::lexicon::app::bsky::feed::get_timeline::main("".to_owned(), "".to_owned(), 32);

    println!("{:?}", res);
}

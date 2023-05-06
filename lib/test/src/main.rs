mod lex;
use lex::lexicon::app::bsky::feed::get_timeline;

use crate::lex::lexicon::com::atproto::server::create_session;

fn main() {
    let session = create_session::main(
        &"".to_owned(),
        create_session::MainInput {
            password: "".to_owned(),
            identifier: "".to_owned(),
        },
    );

    println!("{:?}", session);
    let token = format!("Bearer {}", session.unwrap().access_jwt);
    let res = get_timeline::main(&token, "".to_owned(), "".to_owned(), 32);
    let res = res.unwrap();

    let actor = lex::lexicon::app::bsky::actor::get_profile::main(&token, "".to_owned());
    println!("{:?}", actor);

    for post in res.feed {
        if post.post.viewer.is_some() {
            println!("{:?}", post.post.record);
        }
    }
}

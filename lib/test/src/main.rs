mod lex;
use crate::lex::lexicon::com::atproto::server::create_session;

#[tokio::main]
async fn main() {
    let session = create_session::main(
        &"".to_owned(),
        create_session::MainInput {
            password: "".to_owned(),
            identifier: "".to_owned(),
        },
    )
    .await;

    println!("{:?}", session);
    let session = session.unwrap();
    let token = format!("Bearer {}", session.access_jwt);

    println!("=================");
    let feed = lex::lexicon::app::bsky::feed::get_author_feed::main(
        &token,
        "matrix89.bsky.social".to_string(),
        "".to_string(),
        32,
    )
    .await
    .unwrap();
    println!("{:?}", feed.feed[0]);
}

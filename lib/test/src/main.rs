mod lex;
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use lex::lexicon::app::bsky::feed::get_timeline;
use serde_json::json;

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

    let r = lex::lexicon::com::atproto::sync::subscribe_repos::main(&token, 0).await;

    /*let time: DateTime<Utc> = SystemTime::now().into();
    let post = lex::lexicon::app::bsky::feed::post::Main {
        facets: None,
        reply: None,
        entities: None,
        embed: None,
        text: "Hello from rust \\o/".to_owned(),
        created_at: time.to_rfc3339(),
    };

    let res = lex::lexicon::com::atproto::repo::create_record::main(
        &token,
        lex::lexicon::com::atproto::repo::create_record::MainInput {
            collection: "app.bsky.feed.post".to_owned(),
            repo: session.did.clone(),
            rkey: Some(session.did),
            record: serde_json::json!(&post),
            swap_commit: None,
            validate: Some(true),
        },
    )
    .await
    .unwrap();
    println!("{:?}", res);*/
    /*let res = get_timeline::main(&token, "".to_owned(), "".to_owned(), 32);
    let res = res.await.unwrap();
    let actor = lex::lexicon::app::bsky::actor::get_profile::main(
        &token,
        "matrix89.bsky.social".to_owned(),
    )
    .await;

    println!("{:?}", actor);

    for post in res.feed {
        if post.post.viewer.is_some() {
            println!("{:?}", post.post.record);
        }
    }*/
}

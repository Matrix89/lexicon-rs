mod lex;
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use lex::lexicon::app::bsky::feed::get_timeline;
use serde_json::json;

use crate::lex::lexicon::{
    app::bsky::feed::{self, post},
    com::atproto::{repo::strong_ref, server::create_session},
};

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
    println!("cid: {}", feed.feed[0].post.cid);
    println!("uri: {}", feed.feed[0].post.uri);
    println!("record: {}", feed.feed[0].post.record);
    println!("=================");

    let mut time: DateTime<Utc> = SystemTime::now().into();
    time -= chrono::Duration::hours(2);
    /*
    {
        "collection":"app.bsky.feed.post",
        "repo":"did:plc:d5x67i3gth5jyottsfa56il5",
        "record":{
            "text": "Test",
            "reply": {
                "root": {
                    "uri":"at://did:plc:d5x67i3gth5jyottsfa56il5/app.bsky.feed.post/did:plc:d5x67i3gth5jyottsfa56il5",
                    "cid":"bafyreid6lv4royisreczjp2dxl5kimupekk37quuyyxtycp7bjwvyo366a"
                },
                "parent":{
                    "uri":"at://did:plc:d5x67i3gth5jyottsfa56il5/app.bsky.feed.post/did:plc:d5x67i3gth5jyottsfa56il5",
                    "cid":"bafyreid6lv4royisreczjp2dxl5kimupekk37quuyyxtycp7bjwvyo366a"
                }
            },
            "createdAt":"2023-05-07T21:30:54.643Z",
            "$type":"app.bsky.feed.post"
        }
    }
    */
    let post = post::Main {
        facets: None,
        reply: Some(post::ReplyRef {
            parent: strong_ref::Main {
                cid: "bafyreid6lv4royisreczjp2dxl5kimupekk37quuyyxtycp7bjwvyo366a".to_owned(),
                uri: "at://did:plc:d5x67i3gth5jyottsfa56il5/app.bsky.feed.post/did:plc:d5x67i3gth5jyottsfa56il5".to_owned()
            },
            root: strong_ref::Main {
                cid: "bafyreid6lv4royisreczjp2dxl5kimupekk37quuyyxtycp7bjwvyo366a".to_owned(),
                uri: "at://did:plc:d5x67i3gth5jyottsfa56il5/app.bsky.feed.post/did:plc:d5x67i3gth5jyottsfa56il5".to_owned()
            }
        }),
        entities: None,
        embed: None,
        text: "Sent using https://github.com/Matrix89/lexicon-rs. Is the code ugly? yes, Does it work? somewhat".to_owned(),
        created_at: time.to_rfc3339(),
    };
    /*println!(
        "{:?}",
        serde_json::to_string(
            &lex::lexicon::com::atproto::repo::create_record::MainInput {
                collection: "app.bsky.feed.post".to_owned(),
                repo: session.did.clone(),
                rkey: Some(session.did.clone()),
                record: serde_json::json!(vec![&post]),
                swap_commit: None,
                validate: Some(true),
            }
        ),
    );*/
    let mut record = serde_json::json!(&post).as_object_mut().cloned().unwrap();
    record.insert("$type".to_owned(), json!("app.bsky.feed.post"));
    let input = lex::lexicon::com::atproto::repo::create_record::MainInput {
        collection: "app.bsky.feed.post".to_owned(),
        repo: session.did.clone(),
        rkey: None,
        record: serde_json::json!(record),
        swap_commit: None,
        validate: Some(true),
    };
    println!("{:#?}", input);
    let res = lex::lexicon::com::atproto::repo::create_record::main(&token, input)
        .await
        .unwrap();
    println!("{:?}", res);
    let res = get_timeline::main(&token, "".to_owned(), "".to_owned(), 32);
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
    }
}

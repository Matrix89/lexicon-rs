# Lexicon-rs

WIP

[ATProto's](https://atproto.com/) lexicon to rust - inator

The code is nowhere close to being of any quality as of yet

`cargo run` takes lexicon defintions from a hardcoded patch on my machine
and puts the generated code in `./lib/test/src/lex.rs`

### Creating a session
```rust
let session = create_session::main(
        &"".to_owned(),
        create_session::MainInput {
password: "".to_owned(),
identifier: "".to_owned(),
},
)
.await;

```
### Creating a new record(post in this case)
```rust
let time: DateTime<Utc> = SystemTime::now().into();
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
```
### Getting own timeline
```rust
let res = get_timeline::main(&token, "".to_owned(), "".to_owned(), 32);
let res = res.await.unwrap();
let actor = lex::lexicon::app::bsky::actor::get_profile::main(
        &token,
        "matrix89.bsky.social".to_owned(),
        )
.await;

for post in res.feed {
    if post.post.viewer.is_some() {
        println!("{:?}", post.post.record);
    }
}
```

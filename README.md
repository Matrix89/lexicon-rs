# Lexicon-rs

WIP

(ATProto's)[https://atproto.com/] lexicon to rust - inator

This is the current output for all the lexicons defined (here)[https://github.com/bluesky-social/atproto/tree/main/lexicons]

```rust
pub mod Lexicon {
    pub mod App {
        pub mod Bsky {
            pub mod Actor {
                pub mod Defs {
                    pub struct ProfileViewDetailed {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub followers_count: i64,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub avatar: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub viewer: ViewerState,
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
                        pub did: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub follows_count: i64,
                        #[doc = "format: Handle"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub handle: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub description: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub posts_count: i64,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub labels: Vec<build_ref_target(&r#ref)>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub banner: String,
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub indexed_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub display_name: String,
                    }
                    pub struct ViewerState {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[doc = "format: AtUri"]
                        pub followed_by: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub muted: bool,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub blocked_by: bool,
                        #[doc = "format: AtUri"]
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub blocking: String,
                        #[doc = "nullable: false"]
                        #[doc = "format: AtUri"]
                        #[doc = "required: false"]
                        pub following: String,
                    }
                    pub struct ProfileView {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Datetime"]
                        pub indexed_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub viewer: ViewerState,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub display_name: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub description: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        pub did: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub avatar: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub labels: Vec<build_ref_target(&r#ref)>,
                        #[doc = "format: Handle"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub handle: String,
                    }
                    pub struct ProfileViewBasic {
                        #[doc = "format: Handle"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub handle: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub avatar: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub viewer: ViewerState,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub display_name: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub labels: Vec<build_ref_target(&r#ref)>,
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub did: String,
                    }
                }
                pub mod GetProfile {
                    #[doc = "no desc"]
                    pub fn main(actor: String) -> MainOutput {}
                }
                pub mod GetProfiles {
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub profiles: Vec<build_ref_target(&r#ref)>,
                    }
                    #[doc = "no desc"]
                    pub fn main(actors: Vec<String>) -> MainOutput {}
                }
                pub mod GetSuggestions {
                    pub struct MainOutput {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub actors: Vec<build_ref_target(&r#ref)>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub cursor: String,
                    }
                    #[doc = "Get a list of actors suggested for following. Used in discovery UIs."]
                    pub fn main(cursor: String, limit: i64) -> MainOutput {}
                }
                pub mod Profile {
                    #[doc = "Record {
 description: None, key: Some(\"literal:self\"), record: {
\"properties\": Object {
\"avatar\": Object {
\"accept\": Array [String(\"image/png\"), String(\"image/jpeg\")], \"maxSize\": Number(1000000), \"type\": String(\"blob\")}
, \"banner\": Object {
\"accept\": Array [String(\"image/png\"), String(\"image/jpeg\")], \"maxSize\": Number(1000000), \"type\": String(\"blob\")}
, \"description\": Object {
\"maxGraphemes\": Number(256), \"maxLength\": Number(2560), \"type\": String(\"string\")}
, \"displayName\": Object {
\"maxGraphemes\": Number(64), \"maxLength\": Number(640), \"type\": String(\"string\")}
}
, \"type\": String(\"object\")}
 }
"]
                    pub struct todo_main {}
                }
                pub mod SearchActors {
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub actors: Vec<build_ref_target(&r#ref)>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub cursor: String,
                    }
                    #[doc = "Find actors matching search criteria."]
                    pub fn main(cursor: String, limit: i64, term: String) -> MainOutput {}
                }
                pub mod SearchActorsTypeahead {
                    pub struct MainOutput {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub actors: Vec<build_ref_target(&r#ref)>,
                    }
                    #[doc = "Find actor suggestions for a search term."]
                    pub fn main(term: String, limit: i64) -> MainOutput {}
                }
            }
            pub mod Embed {
                pub mod External {
                    pub struct External {
                        #[doc = "required: true"]
                        #[doc = "format: Uri"]
                        #[doc = "nullable: false"]
                        pub uri: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub description: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub todo2_thumb: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub title: String,
                    }
                    pub struct View {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub external: ViewExternal,
                    }
                    pub struct ViewExternal {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub description: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub thumb: String,
                        #[doc = "nullable: false"]
                        #[doc = "format: Uri"]
                        #[doc = "required: true"]
                        pub uri: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub title: String,
                    }
                    pub struct Main {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub external: External,
                    }
                }
                pub mod Images {
                    pub struct View {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub images: Vec<build_ref_target(&r#ref)>,
                    }
                    pub struct Image {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub todo2_image: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub alt: String,
                    }
                    pub struct Main {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub images: Vec<build_ref_target(&r#ref)>,
                    }
                    pub struct ViewImage {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub thumb: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub alt: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub fullsize: String,
                    }
                }
                pub mod Record {
                    pub struct Main {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub record: Lexicon::Com::Atproto::Repo::StrongRef,
                    }
                    pub struct View {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub todo3_record: String,
                    }
                    pub struct ViewBlocked {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: AtUri"]
                        pub uri: String,
                    }
                    pub struct ViewRecord {
                        #[doc = "required: true"]
                        #[doc = "format: AtUri"]
                        #[doc = "nullable: false"]
                        pub uri: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub labels: Vec<build_ref_target(&r#ref)>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub author: Lexicon::App::Bsky::Actor::Defs::ProfileViewBasic,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub embeds: Vec<String>,
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub indexed_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "format: Cid"]
                        #[doc = "required: true"]
                        pub cid: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub todo4_value: String,
                    }
                    pub struct ViewNotFound {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: AtUri"]
                        pub uri: String,
                    }
                }
                pub mod RecordWithMedia {
                    pub struct Main {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub record: Lexicon::App::Bsky::Embed::Record,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub todo3_media: String,
                    }
                    pub struct View {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub todo3_media: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub record: Lexicon::App::Bsky::Embed::Record::View,
                    }
                }
            }
            pub mod Feed {
                pub mod Defs {
                    pub struct PostView {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub todo4_record: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub viewer: ViewerState,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub labels: Vec<build_ref_target(&r#ref)>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub author: Lexicon::App::Bsky::Actor::Defs::ProfileViewBasic,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Cid"]
                        pub cid: String,
                        #[doc = "nullable: false"]
                        #[doc = "format: AtUri"]
                        #[doc = "required: true"]
                        pub uri: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Datetime"]
                        pub indexed_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub like_count: i64,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub repost_count: i64,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub todo3_embed: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub reply_count: i64,
                    }
                    pub struct ReasonRepost {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub by: Lexicon::App::Bsky::Actor::Defs::ProfileViewBasic,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Datetime"]
                        pub indexed_at: String,
                    }
                    pub struct ReplyRef {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub root: Lexicon::App::Bsky::Feed::Defs::PostView,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub parent: Lexicon::App::Bsky::Feed::Defs::PostView,
                    }
                    pub struct BlockedPost {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub blocked: bool,
                        #[doc = "nullable: false"]
                        #[doc = "format: AtUri"]
                        #[doc = "required: true"]
                        pub uri: String,
                    }
                    pub struct FeedViewPost {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub post: Lexicon::App::Bsky::Feed::Defs::PostView,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub reply: ReplyRef,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub todo3_reason: String,
                    }
                    pub struct NotFoundPost {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: AtUri"]
                        pub uri: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub not_found: bool,
                    }
                    pub struct ThreadViewPost {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub post: PostView,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub todo3_parent: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub replies: Vec<String>,
                    }
                    pub struct ViewerState {
                        #[doc = "format: AtUri"]
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub like: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[doc = "format: AtUri"]
                        pub repost: String,
                    }
                }
                pub mod GetAuthorFeed {
                    pub struct MainOutput {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub feed: Vec<build_ref_target(&r#ref)>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub cursor: String,
                    }
                    #[doc = "A view of an actor's feed."]
                    pub fn main(limit: i64, cursor: String, actor: String) -> MainOutput {}
                }
                pub mod GetLikes {
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub likes: Vec<build_ref_target(&r#ref)>,
                        #[doc = "format: AtUri"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub uri: String,
                        #[doc = "format: Cid"]
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub cid: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub cursor: String,
                    }
                    #[doc = "no desc"]
                    pub fn main(
                        cid: String,
                        uri: String,
                        cursor: String,
                        limit: i64,
                    ) -> MainOutput {
                    }
                    pub struct Like {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Datetime"]
                        pub created_at: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub actor: Lexicon::App::Bsky::Actor::Defs::ProfileView,
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub indexed_at: String,
                    }
                }
                pub mod GetPostThread {
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub todo3_thread: String,
                    }
                    #[doc = "no desc"]
                    pub fn main(depth: i64, uri: String) -> MainOutput {}
                }
                pub mod GetPosts {
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub posts: Vec<build_ref_target(&r#ref)>,
                    }
                    #[doc = "A view of an actor's feed."]
                    pub fn main(uris: Vec<String>) -> MainOutput {}
                }
                pub mod GetRepostedBy {
                    pub struct MainOutput {
                        #[doc = "format: AtUri"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub uri: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub cursor: String,
                        #[doc = "required: false"]
                        #[doc = "format: Cid"]
                        #[doc = "nullable: false"]
                        pub cid: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub reposted_by: Vec<build_ref_target(&r#ref)>,
                    }
                    #[doc = "no desc"]
                    pub fn main(
                        uri: String,
                        cid: String,
                        limit: i64,
                        cursor: String,
                    ) -> MainOutput {
                    }
                }
                pub mod GetTimeline {
                    pub struct MainOutput {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub cursor: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub feed: Vec<build_ref_target(&r#ref)>,
                    }
                    #[doc = "A view of the user's home timeline."]
                    pub fn main(cursor: String, limit: i64, algorithm: String) -> MainOutput {}
                }
                pub mod Like {
                    #[doc = "Record {
 description: None, key: Some(\"tid\"), record: {
\"required\": Array [String(\"subject\"), String(\"createdAt\")], \"properties\": Object {
\"createdAt\": Object {
\"format\": String(\"datetime\"), \"type\": String(\"string\")}
, \"subject\": Object {
\"ref\": String(\"com.atproto.repo.strongRef\"), \"type\": String(\"ref\")}
}
, \"type\": String(\"object\")}
 }
"]
                    pub struct todo_main {}
                }
                pub mod Post {
                    #[doc = "Record {
 description: None, key: Some(\"tid\"), record: {
\"type\": String(\"object\"), \"properties\": Object {
\"createdAt\": Object {
\"format\": String(\"datetime\"), \"type\": String(\"string\")}
, \"embed\": Object {
\"refs\": Array [String(\"app.bsky.embed.images\"), String(\"app.bsky.embed.external\"), String(\"app.bsky.embed.record\"), String(\"app.bsky.embed.recordWithMedia\")], \"type\": String(\"union\")}
, \"entities\": Object {
\"description\": String(\"Deprecated: replaced by app.bsky.richtext.facet.\"), \"items\": Object {
\"ref\": String(\"#entity\"), \"type\": String(\"ref\")}
, \"type\": String(\"array\")}
, \"facets\": Object {
\"items\": Object {
\"ref\": String(\"app.bsky.richtext.facet\"), \"type\": String(\"ref\")}
, \"type\": String(\"array\")}
, \"reply\": Object {
\"ref\": String(\"#replyRef\"), \"type\": String(\"ref\")}
, \"text\": Object {
\"maxGraphemes\": Number(300), \"maxLength\": Number(3000), \"type\": String(\"string\")}
}
, \"required\": Array [String(\"text\"), String(\"createdAt\")]}
 }
"]
                    pub struct todo_main {}
                    pub struct Entity {
                        #[doc = "description: \"Expected values are 'mention' and 'link'.\""]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub r#type: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub index: TextSlice,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub value: String,
                    }
                    pub struct ReplyRef {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub parent: Lexicon::Com::Atproto::Repo::StrongRef,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub root: Lexicon::Com::Atproto::Repo::StrongRef,
                    }
                    pub struct TextSlice {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub end: i64,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub start: i64,
                    }
                }
                pub mod Repost {
                    #[doc = "Record {
 description: None, key: Some(\"tid\"), record: {
\"required\": Array [String(\"subject\"), String(\"createdAt\")], \"properties\": Object {
\"createdAt\": Object {
\"format\": String(\"datetime\"), \"type\": String(\"string\")}
, \"subject\": Object {
\"ref\": String(\"com.atproto.repo.strongRef\"), \"type\": String(\"ref\")}
}
, \"type\": String(\"object\")}
 }
"]
                    pub struct todo_main {}
                }
            }
            pub mod Graph {
                pub mod Block {
                    #[doc = "Record {
 description: Some(\"A block.\"), key: Some(\"tid\"), record: {
\"required\": Array [String(\"subject\"), String(\"createdAt\")], \"properties\": Object {
\"createdAt\": Object {
\"format\": String(\"datetime\"), \"type\": String(\"string\")}
, \"subject\": Object {
\"format\": String(\"did\"), \"type\": String(\"string\")}
}
, \"type\": String(\"object\")}
 }
"]
                    pub struct todo_main {}
                }
                pub mod Follow {
                    #[doc = "Record {
 description: Some(\"A social follow.\"), key: Some(\"tid\"), record: {
\"required\": Array [String(\"subject\"), String(\"createdAt\")], \"properties\": Object {
\"createdAt\": Object {
\"format\": String(\"datetime\"), \"type\": String(\"string\")}
, \"subject\": Object {
\"format\": String(\"did\"), \"type\": String(\"string\")}
}
, \"type\": String(\"object\")}
 }
"]
                    pub struct todo_main {}
                }
                pub mod GetBlocks {
                    pub struct MainOutput {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub cursor: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub blocks: Vec<build_ref_target(&r#ref)>,
                    }
                    #[doc = "Who is the requester's account blocking?"]
                    pub fn main(limit: i64, cursor: String) -> MainOutput {}
                }
                pub mod GetFollowers {
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub followers: Vec<build_ref_target(&r#ref)>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub cursor: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub subject: Lexicon::App::Bsky::Actor::Defs::ProfileView,
                    }
                    #[doc = "Who is following an actor?"]
                    pub fn main(limit: i64, actor: String, cursor: String) -> MainOutput {}
                }
                pub mod GetFollows {
                    pub struct MainOutput {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub subject: Lexicon::App::Bsky::Actor::Defs::ProfileView,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub follows: Vec<build_ref_target(&r#ref)>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub cursor: String,
                    }
                    #[doc = "Who is an actor following?"]
                    pub fn main(cursor: String, limit: i64, actor: String) -> MainOutput {}
                }
                pub mod GetMutes {
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub mutes: Vec<build_ref_target(&r#ref)>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub cursor: String,
                    }
                    #[doc = "Who does the viewer mute?"]
                    pub fn main(cursor: String, limit: i64) -> MainOutput {}
                }
                pub mod MuteActor {
                    pub fn main() {}
                }
                pub mod UnmuteActor {
                    pub fn main() {}
                }
            }
            pub mod Notification {
                pub mod GetUnreadCount {
                    pub struct MainOutput {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub count: i64,
                    }
                    #[doc = "no desc"]
                    pub fn main(seen_at: String) -> MainOutput {}
                }
                pub mod ListNotifications {
                    pub struct MainOutput {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub cursor: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub notifications: Vec<build_ref_target(&r#ref)>,
                    }
                    #[doc = "no desc"]
                    pub fn main(seen_at: String, limit: i64, cursor: String) -> MainOutput {}
                    pub struct Notification {
                        #[doc = "required: true"]
                        #[doc = "format: Cid"]
                        #[doc = "nullable: false"]
                        pub cid: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub is_read: bool,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub author: Lexicon::App::Bsky::Actor::Defs::ProfileView,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub labels: Vec<build_ref_target(&r#ref)>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: AtUri"]
                        pub uri: String,
                        #[doc = "description: \"Expected values are 'like', 'repost', 'follow', 'mention', 'reply', and 'quote'.\""]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub reason: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub todo4_record: String,
                        #[doc = "nullable: false"]
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        pub indexed_at: String,
                        #[doc = "format: AtUri"]
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub reason_subject: String,
                    }
                }
                pub mod UpdateSeen {
                    pub fn main() {}
                }
            }
            pub mod Richtext {
                pub mod Facet {
                    pub struct Mention {
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub did: String,
                    }
                    pub struct Main {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub index: ByteSlice,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub features: Vec<String>,
                    }
                    pub struct Link {
                        #[doc = "required: true"]
                        #[doc = "format: Uri"]
                        #[doc = "nullable: false"]
                        pub uri: String,
                    }
                    pub struct ByteSlice {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub byte_start: i64,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub byte_end: i64,
                    }
                }
            }
            pub mod Unspecced {
                pub mod GetPopular {
                    pub struct MainOutput {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub cursor: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub feed: Vec<build_ref_target(&r#ref)>,
                    }
                    #[doc = "An unspecced view of globally popular items"]
                    pub fn main(cursor: String, limit: i64) -> MainOutput {}
                }
            }
        }
    }
    pub mod Com {
        pub mod Atproto {
            pub mod Admin {
                pub mod Defs {
                    #[doc = "Some(\"Moderation action type: Acknowledge. Indicates that the content was reviewed and not considered to violate PDS rules.\")"]
                    pub struct acknowledge {}
                    pub struct ReportView {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub todo3_subject: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub reason_type: Lexicon::Com::Atproto::Moderation::Defs::ReasonType,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        pub reported_by: String,
                        #[doc = "required: true"]
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        pub created_at: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub resolved_by_action_ids: Vec<i64>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub id: i64,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub reason: String,
                    }
                    #[doc = "Some(\"Moderation action type: Takedown. Indicates that content should not be served by the PDS.\")"]
                    pub struct takedown {}
                    pub struct RepoViewDetail {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Handle"]
                        pub handle: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub moderation: ModerationDetail,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub invited_by: Lexicon::Com::Atproto::Server::Defs::InviteCode,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub labels: Vec<build_ref_target(&r#ref)>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub invites: Vec<build_ref_target(&r#ref)>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub related_records: Vec<String>,
                        #[doc = "required: true"]
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        pub did: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub email: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Datetime"]
                        pub indexed_at: String,
                    }
                    pub struct ModerationDetail {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub reports: Vec<build_ref_target(&r#ref)>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub current_action: ActionViewCurrent,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub actions: Vec<build_ref_target(&r#ref)>,
                    }
                    #[doc = "String {
 description: None, default: None, min_length: None, max_length: None, min_graphemes: None, max_graphemes: None, enum: None, const: None, known_values: None }
"]
                    pub struct todo_actionType {}
                    pub struct ActionViewCurrent {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub action: ActionType,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub id: i64,
                    }
                    pub struct ActionViewDetail {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub todo3_subject: String,
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub created_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub reversal: ActionReversal,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub resolved_reports: Vec<build_ref_target(&r#ref)>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub negate_label_vals: Vec<String>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub id: i64,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub reason: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Did"]
                        pub created_by: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub create_label_vals: Vec<String>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub subject_blobs: Vec<build_ref_target(&r#ref)>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub action: ActionType,
                    }
                    pub struct RepoRef {
                        #[doc = "required: true"]
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        pub did: String,
                    }
                    #[doc = "Some(\"Moderation action type: Flag. Indicates that the content was reviewed and considered to violate PDS rules, but may still be served.\")"]
                    pub struct flag {}
                    pub struct ReportViewDetail {
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub created_at: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub reason_type: Lexicon::Com::Atproto::Moderation::Defs::ReasonType,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Did"]
                        pub reported_by: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub resolved_by_actions: Vec<build_ref_target(&r#ref)>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub reason: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub id: i64,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub todo3_subject: String,
                    }
                    pub struct RecordView {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Cid"]
                        pub cid: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub repo: RepoView,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub todo4_value: String,
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub indexed_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub blob_cids: Vec<String>,
                        #[doc = "format: AtUri"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub uri: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub moderation: Moderation,
                    }
                    pub struct ImageDetails {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub width: i64,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub height: i64,
                    }
                    pub struct RecordViewDetail {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub blobs: Vec<build_ref_target(&r#ref)>,
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub indexed_at: String,
                        #[doc = "required: true"]
                        #[doc = "format: Cid"]
                        #[doc = "nullable: false"]
                        pub cid: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub todo4_value: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: AtUri"]
                        pub uri: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub moderation: ModerationDetail,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub repo: RepoView,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub labels: Vec<build_ref_target(&r#ref)>,
                    }
                    pub struct VideoDetails {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub height: i64,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub length: i64,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub width: i64,
                    }
                    pub struct Moderation {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub current_action: ActionViewCurrent,
                    }
                    pub struct ActionReversal {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Did"]
                        pub created_by: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub reason: String,
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub created_at: String,
                    }
                    pub struct ActionView {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub reversal: ActionReversal,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub action: ActionType,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub reason: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub resolved_report_ids: Vec<i64>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub create_label_vals: Vec<String>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub id: i64,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub negate_label_vals: Vec<String>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub todo3_subject: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub subject_blob_cids: Vec<String>,
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub created_by: String,
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub created_at: String,
                    }
                    pub struct BlobView {
                        #[doc = "required: true"]
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        pub created_at: String,
                        #[doc = "format: Cid"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub cid: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub mime_type: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub todo3_details: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub size: i64,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub moderation: Moderation,
                    }
                    pub struct RepoView {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub moderation: Moderation,
                        #[doc = "nullable: false"]
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        pub indexed_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub invited_by: Lexicon::Com::Atproto::Server::Defs::InviteCode,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub email: String,
                        #[doc = "nullable: false"]
                        #[doc = "format: Handle"]
                        #[doc = "required: true"]
                        pub handle: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub related_records: Vec<String>,
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
                        pub did: String,
                    }
                }
                pub mod DisableInviteCodes {
                    pub fn main() {}
                }
                pub mod GetInviteCodes {
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub codes: Vec<build_ref_target(&r#ref)>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub cursor: String,
                    }
                    #[doc = "Admin view of invite codes"]
                    pub fn main(sort: String, limit: i64, cursor: String) -> MainOutput {}
                }
                pub mod GetModerationAction {
                    #[doc = "View details about a moderation action."]
                    pub fn main(id: i64) -> MainOutput {}
                }
                pub mod GetModerationActions {
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub cursor: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub actions: Vec<build_ref_target(&r#ref)>,
                    }
                    #[doc = "List moderation actions related to a subject."]
                    pub fn main(limit: i64, cursor: String, subject: String) -> MainOutput {}
                }
                pub mod GetModerationReport {
                    #[doc = "View details about a moderation report."]
                    pub fn main(id: i64) -> MainOutput {}
                }
                pub mod GetModerationReports {
                    pub struct MainOutput {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub cursor: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub reports: Vec<build_ref_target(&r#ref)>,
                    }
                    #[doc = "List moderation reports related to a subject."]
                    pub fn main(
                        resolved: bool,
                        cursor: String,
                        subject: String,
                        limit: i64,
                    ) -> MainOutput {
                    }
                }
                pub mod GetRecord {
                    #[doc = "View details about a record."]
                    pub fn main(cid: String, uri: String) -> MainOutput {}
                }
                pub mod GetRepo {
                    #[doc = "View details about a repository."]
                    pub fn main(did: String) -> MainOutput {}
                }
                pub mod ResolveModerationReports {
                    pub fn main() {}
                }
                pub mod ReverseModerationAction {
                    pub fn main() {}
                }
                pub mod SearchRepos {
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub repos: Vec<build_ref_target(&r#ref)>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub cursor: String,
                    }
                    #[doc = "Find repositories based on a search term."]
                    pub fn main(
                        invited_by: String,
                        term: String,
                        cursor: String,
                        limit: i64,
                    ) -> MainOutput {
                    }
                }
                pub mod TakeModerationAction {
                    pub fn main() {}
                }
                pub mod UpdateAccountEmail {
                    pub fn main() {}
                }
                pub mod UpdateAccountHandle {
                    pub fn main() {}
                }
            }
            pub mod Identity {
                pub mod ResolveHandle {
                    pub struct MainOutput {
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub did: String,
                    }
                    #[doc = "Provides the DID of a repo."]
                    pub fn main(handle: String) -> MainOutput {}
                }
                pub mod UpdateHandle {
                    pub fn main() {}
                }
            }
            pub mod Label {
                pub mod Defs {
                    pub struct Label {
                        #[doc = "nullable: false"]
                        #[doc = "description: \"the short string name of the value or type of this label\""]
                        #[doc = "required: true"]
                        pub val: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub neg: bool,
                        #[doc = "description: \"timestamp when this label was created\""]
                        #[doc = "required: true"]
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        pub cts: String,
                        #[doc = "format: Uri"]
                        #[doc = "description: \"AT URI of the record, repository (account), or other resource which this label applies to\""]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub uri: String,
                        #[doc = "format: Cid"]
                        #[doc = "required: false"]
                        #[doc = "description: \"optionally, CID specifying the specific version of 'uri' resource this label applies to\""]
                        #[doc = "nullable: false"]
                        pub cid: String,
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
                        #[doc = "description: \"DID of the actor who created this label\""]
                        pub src: String,
                    }
                }
                pub mod QueryLabels {
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub labels: Vec<build_ref_target(&r#ref)>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub cursor: String,
                    }
                    #[doc = "Find labels relevant to the provided URI patterns."]
                    pub fn main(
                        limit: i64,
                        uri_patterns: Vec<String>,
                        cursor: String,
                        sources: Vec<String>,
                    ) -> MainOutput {
                    }
                }
                pub mod SubscribeLabels {
                    pub struct Labels {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub seq: i64,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub labels: Vec<build_ref_target(&r#ref)>,
                    }
                    pub struct Info {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub message: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub name: String,
                    }
                    #[doc = "Subscribe to label updates"]
                    pub fn main(cursor: i64) {}
                }
            }
            pub mod Moderation {
                pub mod CreateReport {
                    pub fn main() {}
                }
                pub mod Defs {
                    #[doc = "Some(\"Other: reports not falling under another report category\")"]
                    pub struct reasonOther {}
                    #[doc = "String {
 description: None, default: None, min_length: None, max_length: None, min_graphemes: None, max_graphemes: None, enum: None, const: None, known_values: None }
"]
                    pub struct todo_reasonType {}
                    #[doc = "Some(\"Direct violation of server rules, laws, terms of service\")"]
                    pub struct reasonViolation {}
                    #[doc = "Some(\"Spam: frequent unwanted promotion, replies, mentions\")"]
                    pub struct reasonSpam {}
                    #[doc = "Some(\"Unwanted or mis-labeled sexual content\")"]
                    pub struct reasonSexual {}
                    #[doc = "Some(\"Rude, harassing, explicit, or otherwise unwelcoming behavior\")"]
                    pub struct reasonRude {}
                    #[doc = "Some(\"Misleading identity, affiliation, or content\")"]
                    pub struct reasonMisleading {}
                }
            }
            pub mod Repo {
                pub mod ApplyWrites {
                    pub struct Update {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub rkey: String,
                        #[doc = "nullable: false"]
                        #[doc = "format: Nsid"]
                        #[doc = "required: true"]
                        pub collection: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub todo4_value: String,
                    }
                    pub struct Create {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub rkey: String,
                        #[doc = "format: Nsid"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub collection: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub todo4_value: String,
                    }
                    pub struct Delete {
                        #[doc = "nullable: false"]
                        #[doc = "format: Nsid"]
                        #[doc = "required: true"]
                        pub collection: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub rkey: String,
                    }
                    pub fn main() {}
                }
                pub mod CreateRecord {
                    pub fn main() {}
                }
                pub mod DeleteRecord {
                    pub fn main() {}
                }
                pub mod DescribeRepo {
                    pub struct MainOutput {
                        #[doc = "format: Handle"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub handle: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub todo4_did_doc: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub handle_is_correct: bool,
                        #[doc = "required: true"]
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        pub did: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub collections: Vec<String>,
                    }
                    #[doc = "Get information about the repo, including the list of collections."]
                    pub fn main(repo: String) -> MainOutput {}
                }
                pub mod GetRecord {
                    pub struct MainOutput {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: AtUri"]
                        pub uri: String,
                        #[doc = "nullable: false"]
                        #[doc = "format: Cid"]
                        #[doc = "required: false"]
                        pub cid: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub todo4_value: String,
                    }
                    #[doc = "Get a record."]
                    pub fn main(
                        repo: String,
                        cid: String,
                        collection: String,
                        rkey: String,
                    ) -> MainOutput {
                    }
                }
                pub mod ListRecords {
                    pub struct MainOutput {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub cursor: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub records: Vec<build_ref_target(&r#ref)>,
                    }
                    #[doc = "List a range of records in a collection."]
                    pub fn main(
                        collection: String,
                        rkey_end: String,
                        rkey_start: String,
                        reverse: bool,
                        cursor: String,
                        repo: String,
                        limit: i64,
                    ) -> MainOutput {
                    }
                    pub struct Record {
                        #[doc = "format: Cid"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub cid: String,
                        #[doc = "format: AtUri"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub uri: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub todo4_value: String,
                    }
                }
                pub mod PutRecord {
                    pub fn main() {}
                }
                pub mod StrongRef {
                    pub struct Main {
                        #[doc = "format: Cid"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub cid: String,
                        #[doc = "nullable: false"]
                        #[doc = "format: AtUri"]
                        #[doc = "required: true"]
                        pub uri: String,
                    }
                }
                pub mod UploadBlob {
                    pub fn main() {}
                }
            }
            pub mod Server {
                pub mod CreateAccount {
                    pub fn main() {}
                }
                pub mod CreateAppPassword {
                    pub fn main() {}
                    pub struct AppPassword {
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub created_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub password: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub name: String,
                    }
                }
                pub mod CreateInviteCode {
                    pub fn main() {}
                }
                pub mod CreateInviteCodes {
                    pub fn main() {}
                    pub struct AccountCodes {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub codes: Vec<String>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub account: String,
                    }
                }
                pub mod CreateSession {
                    pub fn main() {}
                }
                pub mod Defs {
                    pub struct InviteCodeUse {
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub used_at: String,
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub used_by: String,
                    }
                    pub struct InviteCode {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub disabled: bool,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub code: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub created_by: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub available: i64,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub uses: Vec<build_ref_target(&r#ref)>,
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub created_at: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub for_account: String,
                    }
                }
                pub mod DeleteAccount {
                    pub fn main() {}
                }
                pub mod DeleteSession {
                    pub fn main() {}
                }
                pub mod DescribeServer {
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub available_user_domains: Vec<String>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub links: Links,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub invite_code_required: bool,
                    }
                    #[doc = "Get a document describing the service's accounts configuration."]
                    pub fn main() -> MainOutput {}
                    pub struct Links {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub privacy_policy: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub terms_of_service: String,
                    }
                }
                pub mod GetAccountInviteCodes {
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub codes: Vec<build_ref_target(&r#ref)>,
                    }
                    #[doc = "Get all invite codes for a given account"]
                    pub fn main(include_used: bool, create_available: bool) -> MainOutput {}
                }
                pub mod GetSession {
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Handle"]
                        pub handle: String,
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
                        pub did: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub email: String,
                    }
                    #[doc = "Get information about the current session."]
                    pub fn main() -> MainOutput {}
                }
                pub mod ListAppPasswords {
                    pub struct AppPassword {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub name: String,
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub created_at: String,
                    }
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub passwords: Vec<build_ref_target(&r#ref)>,
                    }
                    #[doc = "List all app-specific passwords."]
                    pub fn main() -> MainOutput {}
                }
                pub mod RefreshSession {
                    pub fn main() {}
                }
                pub mod RequestAccountDelete {
                    pub fn main() {}
                }
                pub mod RequestPasswordReset {
                    pub fn main() {}
                }
                pub mod ResetPassword {
                    pub fn main() {}
                }
                pub mod RevokeAppPassword {
                    pub fn main() {}
                }
            }
            pub mod Sync {
                pub mod GetBlob {
                    #[doc = "Get a blob associated with a given repo."]
                    pub fn main(cid: String, did: String) -> todo {}
                }
                pub mod GetBlocks {
                    #[doc = "Gets blocks from a given repo."]
                    pub fn main(cids: Vec<String>, did: String) -> todo {}
                }
                pub mod GetCheckout {
                    #[doc = "Gets the repo state."]
                    pub fn main(did: String, commit: String) -> todo {}
                }
                pub mod GetCommitPath {
                    pub struct MainOutput {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub commits: Vec<String>,
                    }
                    #[doc = "Gets the path of repo commits"]
                    pub fn main(latest: String, earliest: String, did: String) -> MainOutput {}
                }
                pub mod GetHead {
                    pub struct MainOutput {
                        #[doc = "format: Cid"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub root: String,
                    }
                    #[doc = "Gets the current HEAD CID of a repo."]
                    pub fn main(did: String) -> MainOutput {}
                }
                pub mod GetRecord {
                    #[doc = "Gets blocks needed for existence or non-existence of record."]
                    pub fn main(
                        did: String,
                        collection: String,
                        rkey: String,
                        commit: String,
                    ) -> todo {
                    }
                }
                pub mod GetRepo {
                    #[doc = "Gets the repo state."]
                    pub fn main(latest: String, did: String, earliest: String) -> todo {}
                }
                pub mod ListBlobs {
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub cids: Vec<String>,
                    }
                    #[doc = "List blob cids for some range of commits"]
                    pub fn main(did: String, latest: String, earliest: String) -> MainOutput {}
                }
                pub mod ListRepos {
                    pub struct MainOutput {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub cursor: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub repos: Vec<build_ref_target(&r#ref)>,
                    }
                    #[doc = "List dids and root cids of hosted repos"]
                    pub fn main(cursor: String, limit: i64) -> MainOutput {}
                    pub struct Repo {
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub did: String,
                        #[doc = "required: true"]
                        #[doc = "format: Cid"]
                        #[doc = "nullable: false"]
                        pub head: String,
                    }
                }
                pub mod NotifyOfUpdate {
                    #[doc = "Notify a crawling service of a recent update. Often when a long break between updates causes the connection with the crawling service to break."]
                    pub fn main(hostname: String) -> todo {}
                }
                pub mod RequestCrawl {
                    #[doc = "Request a service to persistently crawl hosted repos."]
                    pub fn main(hostname: String) -> todo {}
                }
                pub mod SubscribeRepos {
                    pub struct Commit {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub todo4_commit: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub ops: Vec<build_ref_target(&r#ref)>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub too_big: bool,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub rebase: bool,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub seq: i64,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        pub repo: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: true"]
                        pub todo4_prev: Option<String>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub blobs: Vec<String>,
                        #[doc = "nullable: false"]
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        pub time: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub todo5_blocks: String,
                    }
                    pub struct RepoOp {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub path: String,
                        #[doc = "nullable: true"]
                        #[doc = "required: true"]
                        pub todo4_cid: Option<String>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub action: String,
                    }
                    pub struct Handle {
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub did: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub seq: i64,
                        #[doc = "nullable: false"]
                        #[doc = "format: Handle"]
                        #[doc = "required: true"]
                        pub handle: String,
                        #[doc = "required: true"]
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        pub time: String,
                    }
                    pub struct Migrate {
                        #[doc = "required: true"]
                        #[doc = "nullable: true"]
                        pub migrate_to: Option<String>,
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub time: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        pub did: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub seq: i64,
                    }
                    pub struct Tombstone {
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub did: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub seq: i64,
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub time: String,
                    }
                    #[doc = "Subscribe to repo updates"]
                    pub fn main(cursor: i64) {}
                    pub struct Info {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub name: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub message: String,
                    }
                }
            }
        }
    }
}
```

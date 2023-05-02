# Lexicon-rs

WIP

[ATProto's](https://atproto.com/) lexicon to rust - inator

The code is nowhere close to being of any quality as of yet

This is the current output for all the lexicons defined [here](https://github.com/bluesky-social/atproto/tree/main/lexicons)

```rust
struct Xrpc {}
pub mod lexicon {
    #[allow(unused_imports)]
    use super::lexicon;
    pub mod app {
        #[allow(unused_imports)]
        use super::lexicon;
        pub mod bsky {
            #[allow(unused_imports)]
            use super::lexicon;
            pub mod actor {
                #[allow(unused_imports)]
                use super::lexicon;
                pub mod defs {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ProfileViewDetailed {
                        pub indexed_at: String,
                        pub avatar: String,
                        pub banner: String,
                        pub handle: String,
                        pub display_name: String,
                        pub description: String,
                        pub followers_count: i64,
                        pub viewer: ViewerState,
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
                        pub did: String,
                        pub posts_count: i64,
                        pub follows_count: i64,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ProfileViewBasic {
                        pub handle: String,
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
                        pub viewer: ViewerState,
                        pub display_name: String,
                        pub avatar: String,
                        pub did: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ViewerState {
                        pub following: String,
                        pub blocking: String,
                        pub followed_by: String,
                        pub blocked_by: bool,
                        pub muted: bool,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ProfileView {
                        pub description: String,
                        pub display_name: String,
                        pub did: String,
                        pub avatar: String,
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
                        pub indexed_at: String,
                        pub viewer: ViewerState,
                        pub handle: String,
                    }
                }
                pub mod get_profile {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    pub fn main(
                        token: &String,
                    ) -> Result<lexicon::app::bsky::actor::defs::ProfileViewDetailed, reqwest::Error>
                    {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/app.bsky.actor.getProfile")
                            .header("Authorization", token)
                            .send()?
                            .json::<lexicon::app::bsky::actor::defs::ProfileViewDetailed>();
                    }
                }
                pub mod get_profiles {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub profiles: Vec<lexicon::app::bsky::actor::defs::ProfileViewDetailed>,
                    }
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/app.bsky.actor.getProfiles")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_suggestions {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub cursor: String,
                        pub actors: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                    }
                    #[doc = "Description: \"Get a list of actors suggested for following. Used in discovery UIs.\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/app.bsky.actor.getSuggestions")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod profile {
                    #[allow(unused_imports)]
                    use super::lexicon;
                }
                pub mod search_actors {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub cursor: String,
                        pub actors: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                    }
                    #[doc = "Description: \"Find actors matching search criteria.\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/app.bsky.actor.searchActors")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod search_actors_typeahead {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub actors: Vec<lexicon::app::bsky::actor::defs::ProfileViewBasic>,
                    }
                    #[doc = "Description: \"Find actor suggestions for a search term.\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/app.bsky.actor.searchActorsTypeahead")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
            }
            pub mod embed {
                #[allow(unused_imports)]
                use super::lexicon;
                pub mod external {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ViewExternal {
                        pub description: String,
                        pub thumb: String,
                        pub uri: String,
                        pub title: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Main {
                        pub external: External,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct View {
                        pub external: ViewExternal,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct External {
                        pub uri: String,
                        pub description: String,
                        pub title: String,
                        pub thumb: String,
                    }
                }
                pub mod images {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ViewImage {
                        pub fullsize: String,
                        pub alt: String,
                        pub thumb: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct View {
                        pub images: Vec<ViewImage>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Main {
                        pub images: Vec<Image>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Image {
                        pub alt: String,
                        pub image: String,
                    }
                }
                pub mod record {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum RecordType {
                        ViewRecord(Box<ViewRecord>),
                        ViewNotFound(Box<ViewNotFound>),
                        ViewBlocked(Box<ViewBlocked>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct View {
                        pub record: RecordType,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ViewBlocked {
                        pub uri: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum EmbedsType {
                        ImagesView,
                        ExternalView,
                        RecordView,
                        RecordWithMediaView,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ViewRecord {
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
                        pub embeds: Vec<EmbedsType>,
                        pub uri: String,
                        pub indexed_at: String,
                        pub cid: String,
                        pub author: lexicon::app::bsky::actor::defs::ProfileViewBasic,
                        pub value: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Main {
                        pub record: lexicon::com::atproto::repo::strong_ref::Main,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ViewNotFound {
                        pub uri: String,
                    }
                }
                pub mod record_with_media {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum MediaType {
                        ImagesView,
                        ExternalView,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct View {
                        pub media: MediaType,
                        pub record: lexicon::app::bsky::embed::record::View,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum MediaType {
                        ImagesMain,
                        ExternalMain,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Main {
                        pub record: lexicon::app::bsky::embed::record::Main,
                        pub media: MediaType,
                    }
                }
            }
            pub mod feed {
                #[allow(unused_imports)]
                use super::lexicon;
                pub mod defs {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum RepliesType {
                        ThreadViewPost(Box<ThreadViewPost>),
                        NotFoundPost(Box<NotFoundPost>),
                        BlockedPost(Box<BlockedPost>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum ParentType {
                        ThreadViewPost(Box<ThreadViewPost>),
                        NotFoundPost(Box<NotFoundPost>),
                        BlockedPost(Box<BlockedPost>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ThreadViewPost {
                        pub replies: Vec<RepliesType>,
                        pub post: PostView,
                        pub parent: ParentType,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum ReasonType {
                        ReasonRepost(Box<ReasonRepost>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct FeedViewPost {
                        pub post: lexicon::app::bsky::feed::defs::PostView,
                        pub reason: ReasonType,
                        pub reply: ReplyRef,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ViewerState {
                        pub repost: String,
                        pub like: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum EmbedType {
                        ImagesView,
                        ExternalView,
                        RecordView,
                        RecordWithMediaView,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct PostView {
                        pub author: lexicon::app::bsky::actor::defs::ProfileViewBasic,
                        pub record: String,
                        pub like_count: i64,
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
                        pub reply_count: i64,
                        pub uri: String,
                        pub repost_count: i64,
                        pub embed: EmbedType,
                        pub indexed_at: String,
                        pub cid: String,
                        pub viewer: ViewerState,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ReasonRepost {
                        pub by: lexicon::app::bsky::actor::defs::ProfileViewBasic,
                        pub indexed_at: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct NotFoundPost {
                        pub uri: String,
                        pub not_found: bool,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ReplyRef {
                        pub parent: lexicon::app::bsky::feed::defs::PostView,
                        pub root: lexicon::app::bsky::feed::defs::PostView,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct BlockedPost {
                        pub uri: String,
                        pub blocked: bool,
                    }
                }
                pub mod get_author_feed {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub cursor: String,
                        pub feed: Vec<lexicon::app::bsky::feed::defs::FeedViewPost>,
                    }
                    #[doc = "Description: \"A view of an actor's feed.\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/app.bsky.feed.getAuthorFeed")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_likes {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Like {
                        pub indexed_at: String,
                        pub created_at: String,
                        pub actor: lexicon::app::bsky::actor::defs::ProfileView,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub cursor: String,
                        pub uri: String,
                        pub cid: String,
                        pub likes: Vec<Like>,
                    }
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/app.bsky.feed.getLikes")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_post_thread {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum ThreadType {
                        DefsThreadViewPost,
                        DefsNotFoundPost,
                        DefsBlockedPost,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub thread: ThreadType,
                    }
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/app.bsky.feed.getPostThread")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_posts {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub posts: Vec<lexicon::app::bsky::feed::defs::PostView>,
                    }
                    #[doc = "Description: \"A view of an actor's feed.\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/app.bsky.feed.getPosts")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_reposted_by {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub cid: String,
                        pub cursor: String,
                        pub uri: String,
                        pub reposted_by: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                    }
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/app.bsky.feed.getRepostedBy")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_timeline {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub feed: Vec<lexicon::app::bsky::feed::defs::FeedViewPost>,
                        pub cursor: String,
                    }
                    #[doc = "Description: \"A view of the user's home timeline.\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/app.bsky.feed.getTimeline")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod like {
                    #[allow(unused_imports)]
                    use super::lexicon;
                }
                pub mod post {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Entity {
                        pub r#type: String,
                        pub index: TextSlice,
                        pub value: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ReplyRef {
                        pub root: lexicon::com::atproto::repo::strong_ref::Main,
                        pub parent: lexicon::com::atproto::repo::strong_ref::Main,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct TextSlice {
                        pub end: i64,
                        pub start: i64,
                    }
                }
                pub mod repost {
                    #[allow(unused_imports)]
                    use super::lexicon;
                }
            }
            pub mod graph {
                #[allow(unused_imports)]
                use super::lexicon;
                pub mod block {
                    #[allow(unused_imports)]
                    use super::lexicon;
                }
                pub mod follow {
                    #[allow(unused_imports)]
                    use super::lexicon;
                }
                pub mod get_blocks {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub cursor: String,
                        pub blocks: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                    }
                    #[doc = "Description: \"Who is the requester's account blocking?\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/app.bsky.graph.getBlocks")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_followers {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub subject: lexicon::app::bsky::actor::defs::ProfileView,
                        pub followers: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                        pub cursor: String,
                    }
                    #[doc = "Description: \"Who is following an actor?\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/app.bsky.graph.getFollowers")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_follows {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub follows: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                        pub cursor: String,
                        pub subject: lexicon::app::bsky::actor::defs::ProfileView,
                    }
                    #[doc = "Description: \"Who is an actor following?\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/app.bsky.graph.getFollows")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_mutes {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub cursor: String,
                        pub mutes: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                    }
                    #[doc = "Description: \"Who does the viewer mute?\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/app.bsky.graph.getMutes")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod mute_actor {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Mute an actor by did or handle.\""]
                    pub fn main(token: &String) -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/app.bsky.graph.muteActor")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod unmute_actor {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Unmute an actor by did or handle.\""]
                    pub fn main(token: &String) -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/app.bsky.graph.unmuteActor")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
            }
            pub mod notification {
                #[allow(unused_imports)]
                use super::lexicon;
                pub mod get_unread_count {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub count: i64,
                    }
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/app.bsky.notification.getUnreadCount")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod list_notifications {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub cursor: String,
                        pub notifications: Vec<Notification>,
                    }
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/app.bsky.notification.listNotifications")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Notification {
                        pub author: lexicon::app::bsky::actor::defs::ProfileView,
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
                        pub reason: String,
                        pub reason_subject: String,
                        pub record: String,
                        pub cid: String,
                        pub is_read: bool,
                        pub indexed_at: String,
                        pub uri: String,
                    }
                }
                pub mod update_seen {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Notify server that the user has seen notifications.\""]
                    pub fn main(token: &String) -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/app.bsky.notification.updateSeen")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
            }
            pub mod richtext {
                #[allow(unused_imports)]
                use super::lexicon;
                pub mod facet {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Link {
                        pub uri: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Mention {
                        pub did: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ByteSlice {
                        pub byte_start: i64,
                        pub byte_end: i64,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum FeaturesType {
                        Mention(Box<Mention>),
                        Link(Box<Link>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Main {
                        pub features: Vec<FeaturesType>,
                        pub index: ByteSlice,
                    }
                }
            }
            pub mod unspecced {
                #[allow(unused_imports)]
                use super::lexicon;
                pub mod get_popular {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub cursor: String,
                        pub feed: Vec<lexicon::app::bsky::feed::defs::FeedViewPost>,
                    }
                    #[doc = "Description: \"An unspecced view of globally popular items\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/app.bsky.unspecced.getPopular")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
            }
        }
    }
    pub mod com {
        #[allow(unused_imports)]
        use super::lexicon;
        pub mod atproto {
            #[allow(unused_imports)]
            use super::lexicon;
            pub mod admin {
                #[allow(unused_imports)]
                use super::lexicon;
                pub mod defs {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ActionReversal {
                        pub created_by: String,
                        pub created_at: String,
                        pub reason: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct VideoDetails {
                        pub width: i64,
                        pub height: i64,
                        pub length: i64,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct RepoRef {
                        pub did: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct RecordView {
                        pub repo: RepoView,
                        pub value: String,
                        pub blob_cids: Vec<String>,
                        pub indexed_at: String,
                        pub cid: String,
                        pub moderation: Moderation,
                        pub uri: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct RepoView {
                        pub related_records: Vec<String>,
                        pub indexed_at: String,
                        pub moderation: Moderation,
                        pub email: String,
                        pub invited_by: lexicon::com::atproto::server::defs::InviteCode,
                        pub handle: String,
                        pub did: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ActionViewCurrent {
                        pub id: i64,
                        pub action: ActionType,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ImageDetails {
                        pub width: i64,
                        pub height: i64,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ModerationDetail {
                        pub current_action: ActionViewCurrent,
                        pub reports: Vec<ReportView>,
                        pub actions: Vec<ActionView>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum SubjectType {
                        RepoView(Box<RepoView>),
                        RecordView(Box<RecordView>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ActionViewDetail {
                        pub reason: String,
                        pub created_at: String,
                        pub subject: SubjectType,
                        pub create_label_vals: Vec<String>,
                        pub resolved_reports: Vec<ReportView>,
                        pub created_by: String,
                        pub action: ActionType,
                        pub id: i64,
                        pub reversal: ActionReversal,
                        pub subject_blobs: Vec<BlobView>,
                        pub negate_label_vals: Vec<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct RecordViewDetail {
                        pub blobs: Vec<BlobView>,
                        pub value: String,
                        pub uri: String,
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
                        pub cid: String,
                        pub indexed_at: String,
                        pub repo: RepoView,
                        pub moderation: ModerationDetail,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Moderation {
                        pub current_action: ActionViewCurrent,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum DetailsType {
                        ImageDetails(Box<ImageDetails>),
                        VideoDetails(Box<VideoDetails>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct BlobView {
                        pub cid: String,
                        pub details: DetailsType,
                        pub size: i64,
                        pub mime_type: String,
                        pub moderation: Moderation,
                        pub created_at: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct RepoViewDetail {
                        pub email: String,
                        pub moderation: ModerationDetail,
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
                        pub invited_by: lexicon::com::atproto::server::defs::InviteCode,
                        pub invites: Vec<lexicon::com::atproto::server::defs::InviteCode>,
                        pub related_records: Vec<String>,
                        pub handle: String,
                        pub indexed_at: String,
                        pub did: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum SubjectType {
                        RepoRef(Box<RepoRef>),
                        StrongRefMain,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ReportView {
                        pub subject: SubjectType,
                        pub reported_by: String,
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                        pub reason: String,
                        pub id: i64,
                        pub created_at: String,
                        pub resolved_by_action_ids: Vec<i64>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum SubjectType {
                        RepoRef(Box<RepoRef>),
                        StrongRefMain,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ActionView {
                        pub create_label_vals: Vec<String>,
                        pub negate_label_vals: Vec<String>,
                        pub reversal: ActionReversal,
                        pub resolved_report_ids: Vec<i64>,
                        pub reason: String,
                        pub created_at: String,
                        pub created_by: String,
                        pub action: ActionType,
                        pub id: i64,
                        pub subject_blob_cids: Vec<String>,
                        pub subject: SubjectType,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum SubjectType {
                        RepoView(Box<RepoView>),
                        RecordView(Box<RecordView>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ReportViewDetail {
                        pub resolved_by_actions:
                            Vec<lexicon::com::atproto::admin::defs::ActionView>,
                        pub created_at: String,
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                        pub id: i64,
                        pub subject: SubjectType,
                        pub reason: String,
                        pub reported_by: String,
                    }
                }
                pub mod disable_invite_codes {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Disable some set of codes and/or all codes associated with a set of users\""]
                    pub fn main(token: &String) -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/com.atproto.admin.disableInviteCodes")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod get_invite_codes {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub codes: Vec<lexicon::com::atproto::server::defs::InviteCode>,
                        pub cursor: String,
                    }
                    #[doc = "Description: \"Admin view of invite codes\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.admin.getInviteCodes")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_moderation_action {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"View details about a moderation action.\""]
                    pub fn main(
                        token: &String,
                    ) -> Result<lexicon::com::atproto::admin::defs::ActionViewDetail, reqwest::Error>
                    {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.admin.getModerationAction")
                            .header("Authorization", token)
                            .send()?
                            .json::<lexicon::com::atproto::admin::defs::ActionViewDetail>();
                    }
                }
                pub mod get_moderation_actions {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub actions: Vec<lexicon::com::atproto::admin::defs::ActionView>,
                        pub cursor: String,
                    }
                    #[doc = "Description: \"List moderation actions related to a subject.\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.admin.getModerationActions")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_moderation_report {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"View details about a moderation report.\""]
                    pub fn main(
                        token: &String,
                    ) -> Result<lexicon::com::atproto::admin::defs::ReportViewDetail, reqwest::Error>
                    {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.admin.getModerationReport")
                            .header("Authorization", token)
                            .send()?
                            .json::<lexicon::com::atproto::admin::defs::ReportViewDetail>();
                    }
                }
                pub mod get_moderation_reports {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub reports: Vec<lexicon::com::atproto::admin::defs::ReportView>,
                        pub cursor: String,
                    }
                    #[doc = "Description: \"List moderation reports related to a subject.\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.admin.getModerationReports")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_record {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"View details about a record.\""]
                    pub fn main(
                        token: &String,
                    ) -> Result<lexicon::com::atproto::admin::defs::RecordViewDetail, reqwest::Error>
                    {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.admin.getRecord")
                            .header("Authorization", token)
                            .send()?
                            .json::<lexicon::com::atproto::admin::defs::RecordViewDetail>();
                    }
                }
                pub mod get_repo {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"View details about a repository.\""]
                    pub fn main(
                        token: &String,
                    ) -> Result<lexicon::com::atproto::admin::defs::RepoViewDetail, reqwest::Error>
                    {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.admin.getRepo")
                            .header("Authorization", token)
                            .send()?
                            .json::<lexicon::com::atproto::admin::defs::RepoViewDetail>();
                    }
                }
                pub mod resolve_moderation_reports {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Resolve moderation reports by an action.\""]
                    pub fn main(
                        token: &String,
                    ) -> Option<lexicon::com::atproto::admin::defs::ActionView>
                    {
                        let client = reqwest::blocking::Client::new();
                        return client . post ("http://bsky.social/xrpc/com.atproto.admin.resolveModerationReports") . header ("Authorization" , token) . send () ? . json :: < lexicon :: com :: atproto :: admin :: defs :: ActionView > () ;
                    }
                }
                pub mod reverse_moderation_action {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Reverse a moderation action.\""]
                    pub fn main(
                        token: &String,
                    ) -> Option<lexicon::com::atproto::admin::defs::ActionView>
                    {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post(
                                "http://bsky.social/xrpc/com.atproto.admin.reverseModerationAction",
                            )
                            .header("Authorization", token)
                            .send()?
                            .json::<lexicon::com::atproto::admin::defs::ActionView>();
                    }
                }
                pub mod search_repos {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub repos: Vec<lexicon::com::atproto::admin::defs::RepoView>,
                        pub cursor: String,
                    }
                    #[doc = "Description: \"Find repositories based on a search term.\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.admin.searchRepos")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod take_moderation_action {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Take a moderation action on a repo.\""]
                    pub fn main(
                        token: &String,
                    ) -> Option<lexicon::com::atproto::admin::defs::ActionView>
                    {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/com.atproto.admin.takeModerationAction")
                            .header("Authorization", token)
                            .send()?
                            .json::<lexicon::com::atproto::admin::defs::ActionView>();
                    }
                }
                pub mod update_account_email {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Administrative action to update an account's email\""]
                    pub fn main(token: &String) -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/com.atproto.admin.updateAccountEmail")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod update_account_handle {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Administrative action to update an account's handle\""]
                    pub fn main(token: &String) -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/com.atproto.admin.updateAccountHandle")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
            }
            pub mod identity {
                #[allow(unused_imports)]
                use super::lexicon;
                pub mod resolve_handle {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub did: String,
                    }
                    #[doc = "Description: \"Provides the DID of a repo.\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.identity.resolveHandle")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod update_handle {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Updates the handle of the account\""]
                    pub fn main(token: &String) -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/com.atproto.identity.updateHandle")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
            }
            pub mod label {
                #[allow(unused_imports)]
                use super::lexicon;
                pub mod defs {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Label {
                        pub src: String,
                        pub cid: String,
                        pub cts: String,
                        pub val: String,
                        pub uri: String,
                        pub neg: bool,
                    }
                }
                pub mod query_labels {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
                        pub cursor: String,
                    }
                    #[doc = "Description: \"Find labels relevant to the provided URI patterns.\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.label.queryLabels")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod subscribe_labels {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Info {
                        pub message: String,
                        pub name: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Labels {
                        pub seq: i64,
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
                    }
                }
            }
            pub mod moderation {
                #[allow(unused_imports)]
                use super::lexicon;
                pub mod create_report {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum SubjectType {
                        DefsRepoRef,
                        StrongRefMain,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Main {
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                        pub reason: String,
                        pub id: i64,
                        pub created_at: String,
                        pub subject: SubjectType,
                        pub reported_by: String,
                    }
                    #[doc = "Description: \"Report a repo or a record.\""]
                    pub fn main(token: &String) -> Option<main> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/com.atproto.moderation.createReport")
                            .header("Authorization", token)
                            .send()?
                            .json::<main>();
                    }
                }
                pub mod defs {
                    #[allow(unused_imports)]
                    use super::lexicon;
                }
            }
            pub mod repo {
                #[allow(unused_imports)]
                use super::lexicon;
                pub mod apply_writes {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Create {
                        pub rkey: String,
                        pub value: String,
                        pub collection: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Delete {
                        pub collection: String,
                        pub rkey: String,
                    }
                    #[doc = "Description: \"Apply a batch transaction of creates, updates, and deletes.\""]
                    pub fn main(token: &String) -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/com.atproto.repo.applyWrites")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Update {
                        pub rkey: String,
                        pub value: String,
                        pub collection: String,
                    }
                }
                pub mod create_record {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Main {
                        pub uri: String,
                        pub cid: String,
                    }
                    #[doc = "Description: \"Create a new record.\""]
                    pub fn main(token: &String) -> Option<main> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/com.atproto.repo.createRecord")
                            .header("Authorization", token)
                            .send()?
                            .json::<main>();
                    }
                }
                pub mod delete_record {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Delete a record, or ensure it doesn't exist.\""]
                    pub fn main(token: &String) -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/com.atproto.repo.deleteRecord")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod describe_repo {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub handle: String,
                        pub collections: Vec<String>,
                        pub did: String,
                        pub handle_is_correct: bool,
                        pub didDoc: String,
                    }
                    #[doc = "Description: \"Get information about the repo, including the list of collections.\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.repo.describeRepo")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_record {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub cid: String,
                        pub uri: String,
                        pub value: String,
                    }
                    #[doc = "Description: \"Get a record.\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.repo.getRecord")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod list_records {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Record {
                        pub cid: String,
                        pub value: String,
                        pub uri: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub records: Vec<Record>,
                        pub cursor: String,
                    }
                    #[doc = "Description: \"List a range of records in a collection.\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.repo.listRecords")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod put_record {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Main {
                        pub cid: String,
                        pub uri: String,
                    }
                    #[doc = "Description: \"Write a record, creating or updating it as needed.\""]
                    pub fn main(token: &String) -> Option<main> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/com.atproto.repo.putRecord")
                            .header("Authorization", token)
                            .send()?
                            .json::<main>();
                    }
                }
                pub mod strong_ref {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Main {
                        pub uri: String,
                        pub cid: String,
                    }
                }
                pub mod upload_blob {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Main {
                        pub blob: String,
                    }
                    #[doc = "Description: \"Upload a new blob to be added to repo in a later request.\""]
                    pub fn main(token: &String) -> Option<main> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/com.atproto.repo.uploadBlob")
                            .header("Authorization", token)
                            .send()?
                            .json::<main>();
                    }
                }
            }
            pub mod server {
                #[allow(unused_imports)]
                use super::lexicon;
                pub mod create_account {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Main {
                        pub refresh_jwt: String,
                        pub did: String,
                        pub access_jwt: String,
                        pub handle: String,
                    }
                    #[doc = "Description: \"Create an account.\""]
                    pub fn main(token: &String) -> Option<main> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/com.atproto.server.createAccount")
                            .header("Authorization", token)
                            .send()?
                            .json::<main>();
                    }
                }
                pub mod create_app_password {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct AppPassword {
                        pub created_at: String,
                        pub password: String,
                        pub name: String,
                    }
                    #[doc = "Description: \"Create an app-specific password.\""]
                    pub fn main(token: &String) -> Option<AppPassword> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/com.atproto.server.createAppPassword")
                            .header("Authorization", token)
                            .send()?
                            .json::<AppPassword>();
                    }
                }
                pub mod create_invite_code {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Main {
                        pub code: String,
                    }
                    #[doc = "Description: \"Create an invite code.\""]
                    pub fn main(token: &String) -> Option<main> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/com.atproto.server.createInviteCode")
                            .header("Authorization", token)
                            .send()?
                            .json::<main>();
                    }
                }
                pub mod create_invite_codes {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Main {
                        pub codes: Vec<AccountCodes>,
                    }
                    #[doc = "Description: \"Create an invite code.\""]
                    pub fn main(token: &String) -> Option<main> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/com.atproto.server.createInviteCodes")
                            .header("Authorization", token)
                            .send()?
                            .json::<main>();
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct AccountCodes {
                        pub account: String,
                        pub codes: Vec<String>,
                    }
                }
                pub mod create_session {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Main {
                        pub email: String,
                        pub access_jwt: String,
                        pub refresh_jwt: String,
                        pub handle: String,
                        pub did: String,
                    }
                    #[doc = "Description: \"Create an authentication session.\""]
                    pub fn main(token: &String) -> Option<main> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/com.atproto.server.createSession")
                            .header("Authorization", token)
                            .send()?
                            .json::<main>();
                    }
                }
                pub mod defs {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct InviteCodeUse {
                        pub used_by: String,
                        pub used_at: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct InviteCode {
                        pub for_account: String,
                        pub disabled: bool,
                        pub code: String,
                        pub created_by: String,
                        pub available: i64,
                        pub uses: Vec<InviteCodeUse>,
                        pub created_at: String,
                    }
                }
                pub mod delete_account {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Delete a user account with a token and password.\""]
                    pub fn main(token: &String) -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/com.atproto.server.deleteAccount")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod delete_session {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Delete the current session.\""]
                    pub fn main(token: &String) -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/com.atproto.server.deleteSession")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod describe_server {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub invite_code_required: bool,
                        pub links: Links,
                        pub available_user_domains: Vec<String>,
                    }
                    #[doc = "Description: \"Get a document describing the service's accounts configuration.\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.server.describeServer")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Links {
                        pub privacy_policy: String,
                        pub terms_of_service: String,
                    }
                }
                pub mod get_account_invite_codes {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub codes: Vec<lexicon::com::atproto::server::defs::InviteCode>,
                    }
                    #[doc = "Description: \"Get all invite codes for a given account\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.server.getAccountInviteCodes")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_session {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub handle: String,
                        pub email: String,
                        pub did: String,
                    }
                    #[doc = "Description: \"Get information about the current session.\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.server.getSession")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod list_app_passwords {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub passwords: Vec<AppPassword>,
                    }
                    #[doc = "Description: \"List all app-specific passwords.\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.server.listAppPasswords")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct AppPassword {
                        pub name: String,
                        pub created_at: String,
                    }
                }
                pub mod refresh_session {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Main {
                        pub access_jwt: String,
                        pub handle: String,
                        pub refresh_jwt: String,
                        pub did: String,
                    }
                    #[doc = "Description: \"Refresh an authentication session.\""]
                    pub fn main(token: &String) -> Option<main> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/com.atproto.server.refreshSession")
                            .header("Authorization", token)
                            .send()?
                            .json::<main>();
                    }
                }
                pub mod request_account_delete {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Initiate a user account deletion via email.\""]
                    pub fn main(token: &String) -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/com.atproto.server.requestAccountDelete")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod request_password_reset {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Initiate a user account password reset via email.\""]
                    pub fn main(token: &String) -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/com.atproto.server.requestPasswordReset")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod reset_password {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Reset a user account password using a token.\""]
                    pub fn main(token: &String) -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/com.atproto.server.resetPassword")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod revoke_app_password {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Revoke an app-specific password by name.\""]
                    pub fn main(token: &String) -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("http://bsky.social/xrpc/com.atproto.server.revokeAppPassword")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
            }
            pub mod sync {
                #[allow(unused_imports)]
                use super::lexicon;
                pub mod get_blob {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Get a blob associated with a given repo.\""]
                    pub fn main(token: &String) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.sync.getBlob")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod get_blocks {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Gets blocks from a given repo.\""]
                    pub fn main(token: &String) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.sync.getBlocks")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod get_checkout {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Gets the repo state.\""]
                    pub fn main(token: &String) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.sync.getCheckout")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod get_commit_path {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub commits: Vec<String>,
                    }
                    #[doc = "Description: \"Gets the path of repo commits\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.sync.getCommitPath")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_head {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub root: String,
                    }
                    #[doc = "Description: \"Gets the current HEAD CID of a repo.\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.sync.getHead")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_record {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Gets blocks needed for existence or non-existence of record.\""]
                    pub fn main(token: &String) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.sync.getRecord")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod get_repo {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Gets the repo state.\""]
                    pub fn main(token: &String) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.sync.getRepo")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod list_blobs {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub cids: Vec<String>,
                    }
                    #[doc = "Description: \"List blob cids for some range of commits\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.sync.listBlobs")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod list_repos {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Repo {
                        pub head: String,
                        pub did: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub cursor: String,
                        pub repos: Vec<Repo>,
                    }
                    #[doc = "Description: \"List dids and root cids of hosted repos\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.sync.listRepos")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod notify_of_update {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Notify a crawling service of a recent update. Often when a long break between updates causes the connection with the crawling service to break.\""]
                    pub fn main(token: &String) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.sync.notifyOfUpdate")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod request_crawl {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Request a service to persistently crawl hosted repos.\""]
                    pub fn main(token: &String) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.sync.requestCrawl")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod subscribe_repos {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct RepoOp {
                        pub cid: Option<String>,
                        pub action: String,
                        pub path: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Handle {
                        pub did: String,
                        pub seq: i64,
                        pub handle: String,
                        pub time: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Commit {
                        pub ops: Vec<RepoOp>,
                        pub blobs: Vec<String>,
                        pub repo: String,
                        pub rebase: bool,
                        pub seq: i64,
                        pub too_big: bool,
                        pub blocks: String,
                        pub time: String,
                        pub prev: Option<String>,
                        pub commit: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Migrate {
                        pub time: String,
                        pub did: String,
                        pub migrate_to: Option<String>,
                        pub seq: i64,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Tombstone {
                        pub seq: i64,
                        pub did: String,
                        pub time: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Info {
                        pub message: String,
                        pub name: String,
                    }
                }
            }
        }
    }
}
```

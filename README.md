# Lexicon-rs

WIP

[ATProto's](https://atproto.com/) lexicon to rust - inator

The code is nowhere close to being of any quality as of yet

This is the current output for all the lexicons defined [here](https://github.com/bluesky-social/atproto/tree/main/lexicons)

```rust
pub mod Lexicon {
    pub mod App {
        pub mod Bsky {
            pub mod Actor {
                pub mod Defs {
                    #[doc = ""]
                    pub struct ViewerState {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub muted: bool,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub blocked_by: bool,
                        #[doc = "format: AtUri"]
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub blocking: String,
                        #[doc = "format: AtUri"]
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub followed_by: String,
                        #[doc = "format: AtUri"]
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub following: String,
                    }
                    #[doc = ""]
                    pub struct ProfileViewDetailed {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub viewer: ViewerState,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub postsCount: i64,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub display_name: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub labels: Vec<crate::Lexicon::Com::Atproto::Label::Defs::Label>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub followersCount: i64,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub description: String,
                        #[doc = "required: false"]
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        pub indexed_at: String,
                        #[doc = "required: true"]
                        #[doc = "format: Handle"]
                        #[doc = "nullable: false"]
                        pub handle: String,
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
                        pub did: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub followsCount: i64,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub avatar: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub banner: String,
                    }
                    #[doc = ""]
                    pub struct ProfileViewBasic {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub display_name: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Did"]
                        pub did: String,
                        #[doc = "format: Handle"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub handle: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub avatar: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub viewer: ViewerState,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub labels: Vec<crate::Lexicon::Com::Atproto::Label::Defs::Label>,
                    }
                    #[doc = ""]
                    pub struct ProfileView {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub description: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub avatar: String,
                        #[doc = "required: true"]
                        #[doc = "format: Handle"]
                        #[doc = "nullable: false"]
                        pub handle: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub display_name: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub viewer: ViewerState,
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub indexed_at: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub labels: Vec<crate::Lexicon::Com::Atproto::Label::Defs::Label>,
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub did: String,
                    }
                }
                pub mod GetProfile {
                    pub fn main(
                        actor: String,
                    ) -> Option<crate::Lexicon::App::Bsky::Actor::Defs::ProfileViewDetailed>
                    {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/app/bsky/actor/getProfile");
                        return None;
                    }
                }
                pub mod GetProfiles {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub profiles:
                            Vec<crate::Lexicon::App::Bsky::Actor::Defs::ProfileViewDetailed>,
                    }
                    pub fn main(actors: Vec<String>) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/app/bsky/actor/getProfiles");
                        return None;
                    }
                }
                pub mod GetSuggestions {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub cursor: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub actors: Vec<crate::Lexicon::App::Bsky::Actor::Defs::ProfileView>,
                    }
                    #[doc = "Description: \"Get a list of actors suggested for following. Used in discovery UIs.\""]
                    pub fn main(limit: i64, cursor: String) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/app/bsky/actor/getSuggestions");
                        return None;
                    }
                }
                pub mod Profile {}
                pub mod SearchActors {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub cursor: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub actors: Vec<crate::Lexicon::App::Bsky::Actor::Defs::ProfileView>,
                    }
                    #[doc = "Description: \"Find actors matching search criteria.\""]
                    pub fn main(limit: i64, cursor: String, term: String) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/app/bsky/actor/searchActors");
                        return None;
                    }
                }
                pub mod SearchActorsTypeahead {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub actors: Vec<crate::Lexicon::App::Bsky::Actor::Defs::ProfileViewBasic>,
                    }
                    #[doc = "Description: \"Find actor suggestions for a search term.\""]
                    pub fn main(limit: i64, term: String) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client
                            .get("http://localhost:8080/xrpc/app/bsky/actor/searchActorsTypeahead");
                        return None;
                    }
                }
            }
            pub mod Embed {
                pub mod External {
                    #[doc = ""]
                    pub struct External {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub title: String,
                        #[doc = "required: true"]
                        #[doc = "format: Uri"]
                        #[doc = "nullable: false"]
                        pub uri: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[doc = "accept: [\"image/*\"]"]
                        pub thumb: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub description: String,
                    }
                    #[doc = ""]
                    pub struct ViewExternal {
                        #[doc = "required: true"]
                        #[doc = "format: Uri"]
                        #[doc = "nullable: false"]
                        pub uri: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub description: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub thumb: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub title: String,
                    }
                    #[doc = ""]
                    pub struct View {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub external: ViewExternal,
                    }
                    #[doc = ""]
                    pub struct Main {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub external: External,
                    }
                }
                pub mod Images {
                    #[doc = ""]
                    pub struct View {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub images: Vec<ViewImage>,
                    }
                    #[doc = ""]
                    pub struct Main {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub images: Vec<Image>,
                    }
                    #[doc = ""]
                    pub struct Image {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "accept: [\"image/*\"]"]
                        pub image: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub alt: String,
                    }
                    #[doc = ""]
                    pub struct ViewImage {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub fullsize: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub alt: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub thumb: String,
                    }
                }
                pub mod Record {
                    #[doc = ""]
                    pub enum EmbedsType {
                        ImagesView,
                        ExternalView,
                        RecordView,
                        RecordWithMediaView,
                    }
                    pub struct ViewRecord {
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub indexed_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Cid"]
                        pub cid: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub todo4_value: String,
                        #[doc = "format: AtUri"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub uri: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub author: crate::Lexicon::App::Bsky::Actor::Defs::ProfileViewBasic,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub embeds: Vec<EmbedsType>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub labels: Vec<crate::Lexicon::Com::Atproto::Label::Defs::Label>,
                    }
                    #[doc = ""]
                    pub struct ViewNotFound {
                        #[doc = "format: AtUri"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub uri: String,
                    }
                    #[doc = ""]
                    pub struct Main {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub record: crate::Lexicon::Com::Atproto::Repo::StrongRef::Main,
                    }
                    #[doc = ""]
                    pub enum ViewrecordType {
                        ViewRecord(Box<ViewRecord>),
                        ViewNotFound(Box<ViewNotFound>),
                        ViewBlocked(Box<ViewBlocked>),
                    }
                    pub struct View {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub record: ViewrecordType,
                    }
                    #[doc = ""]
                    pub struct ViewBlocked {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: AtUri"]
                        pub uri: String,
                    }
                }
                pub mod RecordWithMedia {
                    #[doc = ""]
                    pub enum MainmediaType {
                        ImagesMain,
                        ExternalMain,
                    }
                    pub struct Main {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub record: crate::Lexicon::App::Bsky::Embed::Record::Main,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub media: MainmediaType,
                    }
                    #[doc = ""]
                    pub enum ViewmediaType {
                        ImagesView,
                        ExternalView,
                    }
                    pub struct View {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub record: crate::Lexicon::App::Bsky::Embed::Record::View,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub media: ViewmediaType,
                    }
                }
            }
            pub mod Feed {
                pub mod Defs {
                    #[doc = ""]
                    pub struct ReasonRepost {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub by: crate::Lexicon::App::Bsky::Actor::Defs::ProfileViewBasic,
                        #[doc = "required: true"]
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        pub indexed_at: String,
                    }
                    #[doc = ""]
                    pub enum PostViewembedType {
                        ImagesView,
                        ExternalView,
                        RecordView,
                        RecordWithMediaView,
                    }
                    pub struct PostView {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub todo4_record: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub repostCount: i64,
                        #[doc = "format: AtUri"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub uri: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub author: crate::Lexicon::App::Bsky::Actor::Defs::ProfileViewBasic,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Datetime"]
                        pub indexed_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub labels: Vec<crate::Lexicon::Com::Atproto::Label::Defs::Label>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub replyCount: i64,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub viewer: ViewerState,
                        #[doc = "required: true"]
                        #[doc = "format: Cid"]
                        #[doc = "nullable: false"]
                        pub cid: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub embed: PostViewembedType,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub likeCount: i64,
                    }
                    #[doc = ""]
                    pub struct BlockedPost {
                        #[doc = "format: AtUri"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub uri: String,
                        #[doc = "const: true"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub blocked: bool,
                    }
                    #[doc = ""]
                    pub enum ThreadViewPostparentType {
                        ThreadViewPost(Box<ThreadViewPost>),
                        NotFoundPost(Box<NotFoundPost>),
                        BlockedPost(Box<BlockedPost>),
                    }
                    pub enum RepliesType {
                        ThreadViewPost(Box<ThreadViewPost>),
                        NotFoundPost(Box<NotFoundPost>),
                        BlockedPost(Box<BlockedPost>),
                    }
                    pub struct ThreadViewPost {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub post: PostView,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub parent: ThreadViewPostparentType,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub replies: Vec<RepliesType>,
                    }
                    #[doc = ""]
                    pub struct NotFoundPost {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "const: true"]
                        pub not_found: bool,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: AtUri"]
                        pub uri: String,
                    }
                    #[doc = ""]
                    pub struct ReplyRef {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub parent: crate::Lexicon::App::Bsky::Feed::Defs::PostView,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub root: crate::Lexicon::App::Bsky::Feed::Defs::PostView,
                    }
                    #[doc = ""]
                    pub enum FeedViewPostreasonType {
                        ReasonRepost(Box<ReasonRepost>),
                    }
                    pub struct FeedViewPost {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub reply: ReplyRef,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub reason: FeedViewPostreasonType,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub post: crate::Lexicon::App::Bsky::Feed::Defs::PostView,
                    }
                    #[doc = ""]
                    pub struct ViewerState {
                        #[doc = "format: AtUri"]
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub like: String,
                        #[doc = "format: AtUri"]
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub repost: String,
                    }
                }
                pub mod GetAuthorFeed {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub feed: Vec<crate::Lexicon::App::Bsky::Feed::Defs::FeedViewPost>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub cursor: String,
                    }
                    #[doc = "Description: \"A view of an actor's feed.\""]
                    pub fn main(cursor: String, actor: String, limit: i64) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/app/bsky/feed/getAuthorFeed");
                        return None;
                    }
                }
                pub mod GetLikes {
                    #[doc = ""]
                    pub struct Like {
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub indexed_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        pub created_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub actor: crate::Lexicon::App::Bsky::Actor::Defs::ProfileView,
                    }
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub cursor: String,
                        #[doc = "nullable: false"]
                        #[doc = "format: Cid"]
                        #[doc = "required: false"]
                        pub cid: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: AtUri"]
                        pub uri: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub likes: Vec<Like>,
                    }
                    pub fn main(
                        limit: i64,
                        uri: String,
                        cid: String,
                        cursor: String,
                    ) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/app/bsky/feed/getLikes");
                        return None;
                    }
                }
                pub mod GetPostThread {
                    #[doc = ""]
                    pub enum MainOutputthreadType {
                        DefsThreadViewPost,
                        DefsNotFoundPost,
                        DefsBlockedPost,
                    }
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub thread: MainOutputthreadType,
                    }
                    pub fn main(depth: i64, uri: String) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/app/bsky/feed/getPostThread");
                        return None;
                    }
                }
                pub mod GetPosts {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub posts: Vec<crate::Lexicon::App::Bsky::Feed::Defs::PostView>,
                    }
                    #[doc = "Description: \"A view of an actor's feed.\""]
                    pub fn main(uris: Vec<String>) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/app/bsky/feed/getPosts");
                        return None;
                    }
                }
                pub mod GetRepostedBy {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub reposted_by: Vec<crate::Lexicon::App::Bsky::Actor::Defs::ProfileView>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub cursor: String,
                        #[doc = "format: Cid"]
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub cid: String,
                        #[doc = "format: AtUri"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub uri: String,
                    }
                    pub fn main(
                        limit: i64,
                        cid: String,
                        uri: String,
                        cursor: String,
                    ) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/app/bsky/feed/getRepostedBy");
                        return None;
                    }
                }
                pub mod GetTimeline {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub cursor: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub feed: Vec<crate::Lexicon::App::Bsky::Feed::Defs::FeedViewPost>,
                    }
                    #[doc = "Description: \"A view of the user's home timeline.\""]
                    pub fn main(
                        limit: i64,
                        cursor: String,
                        algorithm: String,
                    ) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/app/bsky/feed/getTimeline");
                        return None;
                    }
                }
                pub mod Like {}
                pub mod Post {
                    #[doc = "Deprecated. Use app.bsky.richtext instead -- A text segment. Start is inclusive, end is exclusive. Indices are for utf16-encoded strings."]
                    pub struct TextSlice {
                        #[doc = "minimum: 0"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub end: i64,
                        #[doc = "required: true"]
                        #[doc = "minimum: 0"]
                        #[doc = "nullable: false"]
                        pub start: i64,
                    }
                    #[doc = ""]
                    pub struct ReplyRef {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub root: crate::Lexicon::Com::Atproto::Repo::StrongRef::Main,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub parent: crate::Lexicon::Com::Atproto::Repo::StrongRef::Main,
                    }
                    #[doc = "Deprecated: use facets instead."]
                    pub struct Entity {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub index: TextSlice,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub value: String,
                        #[doc = "nullable: false"]
                        #[doc = "description: \"Expected values are 'mention' and 'link'.\""]
                        #[doc = "required: true"]
                        pub r#type: String,
                    }
                }
                pub mod Repost {}
            }
            pub mod Graph {
                pub mod Block {}
                pub mod Follow {}
                pub mod GetBlocks {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub blocks: Vec<crate::Lexicon::App::Bsky::Actor::Defs::ProfileView>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub cursor: String,
                    }
                    #[doc = "Description: \"Who is the requester's account blocking?\""]
                    pub fn main(limit: i64, cursor: String) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/app/bsky/graph/getBlocks");
                        return None;
                    }
                }
                pub mod GetFollowers {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub followers: Vec<crate::Lexicon::App::Bsky::Actor::Defs::ProfileView>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub cursor: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub subject: crate::Lexicon::App::Bsky::Actor::Defs::ProfileView,
                    }
                    #[doc = "Description: \"Who is following an actor?\""]
                    pub fn main(cursor: String, actor: String, limit: i64) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/app/bsky/graph/getFollowers");
                        return None;
                    }
                }
                pub mod GetFollows {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub follows: Vec<crate::Lexicon::App::Bsky::Actor::Defs::ProfileView>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub subject: crate::Lexicon::App::Bsky::Actor::Defs::ProfileView,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub cursor: String,
                    }
                    #[doc = "Description: \"Who is an actor following?\""]
                    pub fn main(limit: i64, cursor: String, actor: String) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/app/bsky/graph/getFollows");
                        return None;
                    }
                }
                pub mod GetMutes {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub cursor: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub mutes: Vec<crate::Lexicon::App::Bsky::Actor::Defs::ProfileView>,
                    }
                    #[doc = "Description: \"Who does the viewer mute?\""]
                    pub fn main(cursor: String, limit: i64) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/app/bsky/graph/getMutes");
                        return None;
                    }
                }
                pub mod MuteActor {
                    #[doc = "Description: \"Mute an actor by did or handle.\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/app/bsky/graph/muteActor");
                        return None;
                    }
                }
                pub mod UnmuteActor {
                    #[doc = "Description: \"Unmute an actor by did or handle.\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/app/bsky/graph/unmuteActor");
                        return None;
                    }
                }
            }
            pub mod Notification {
                pub mod GetUnreadCount {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub count: i64,
                    }
                    pub fn main(seen_at: String) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client
                            .get("http://localhost:8080/xrpc/app/bsky/notification/getUnreadCount");
                        return None;
                    }
                }
                pub mod ListNotifications {
                    #[doc = ""]
                    pub enum Reason {
                        Like,
                        Repost,
                        Follow,
                        Mention,
                        Reply,
                        Quote,
                    }
                    pub struct Notification {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Cid"]
                        pub cid: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub todo4_record: String,
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub indexed_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub is_read: bool,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: AtUri"]
                        pub uri: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "description: \"Expected values are 'like', 'repost', 'follow', 'mention', 'reply', and 'quote'.\""]
                        #[doc = "known_values: [\"like\", \"repost\", \"follow\", \"mention\", \"reply\", \"quote\"]"]
                        pub reason: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub labels: Vec<crate::Lexicon::Com::Atproto::Label::Defs::Label>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub author: crate::Lexicon::App::Bsky::Actor::Defs::ProfileView,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[doc = "format: AtUri"]
                        pub reason_subject: String,
                    }
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub cursor: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub notifications: Vec<Notification>,
                    }
                    pub fn main(limit: i64, cursor: String, seen_at: String) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get(
                            "http://localhost:8080/xrpc/app/bsky/notification/listNotifications",
                        );
                        return None;
                    }
                }
                pub mod UpdateSeen {
                    #[doc = "Description: \"Notify server that the user has seen notifications.\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/app/bsky/notification/updateSeen");
                        return None;
                    }
                }
            }
            pub mod Richtext {
                pub mod Facet {
                    #[doc = ""]
                    pub enum FeaturesType {
                        Mention(Box<Mention>),
                        Link(Box<Link>),
                    }
                    pub struct Main {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub index: ByteSlice,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub features: Vec<FeaturesType>,
                    }
                    #[doc = "A text segment. Start is inclusive, end is exclusive. Indices are for utf8-encoded strings."]
                    pub struct ByteSlice {
                        #[doc = "minimum: 0"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub byteStart: i64,
                        #[doc = "minimum: 0"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub byteEnd: i64,
                    }
                    #[doc = "A facet feature for actor mentions."]
                    pub struct Mention {
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub did: String,
                    }
                    #[doc = "A facet feature for links."]
                    pub struct Link {
                        #[doc = "nullable: false"]
                        #[doc = "format: Uri"]
                        #[doc = "required: true"]
                        pub uri: String,
                    }
                }
            }
            pub mod Unspecced {
                pub mod GetPopular {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub cursor: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub feed: Vec<crate::Lexicon::App::Bsky::Feed::Defs::FeedViewPost>,
                    }
                    #[doc = "Description: \"An unspecced view of globally popular items\""]
                    pub fn main(limit: i64, cursor: String) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/app/bsky/unspecced/getPopular");
                        return None;
                    }
                }
            }
        }
    }
    pub mod Com {
        pub mod Atproto {
            pub mod Admin {
                pub mod Defs {
                    #[doc = ""]
                    pub struct RepoView {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub related_records: Vec<String>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub moderation: Moderation,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub invited_by: crate::Lexicon::Com::Atproto::Server::Defs::InviteCode,
                        #[doc = "format: Handle"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub handle: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub email: String,
                        #[doc = "required: true"]
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        pub indexed_at: String,
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub did: String,
                    }
                    #[doc = ""]
                    pub struct RecordViewDetail {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub blobs: Vec<BlobView>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub repo: RepoView,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub todo4_value: String,
                        #[doc = "format: AtUri"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub uri: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub labels: Vec<crate::Lexicon::Com::Atproto::Label::Defs::Label>,
                        #[doc = "format: Cid"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub cid: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub moderation: ModerationDetail,
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub indexed_at: String,
                    }
                    #[doc = ""]
                    pub enum ActionViewDetailsubjectType {
                        RepoView(Box<RepoView>),
                        RecordView(Box<RecordView>),
                    }
                    pub struct ActionViewDetail {
                        #[doc = "required: true"]
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        pub created_by: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub create_label_vals: Vec<String>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub reason: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub reversal: ActionReversal,
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub created_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub id: i64,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub action: ActionType,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub subject_blobs: Vec<BlobView>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub negate_label_vals: Vec<String>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub resolved_reports: Vec<ReportView>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub subject: ActionViewDetailsubjectType,
                    }
                    #[doc = ""]
                    pub enum ReportViewsubjectType {
                        RepoRef(Box<RepoRef>),
                        StrongRefMain,
                    }
                    pub struct ReportView {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub id: i64,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub subject: ReportViewsubjectType,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub reason: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Datetime"]
                        pub created_at: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub resolved_by_action_ids: Vec<i64>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub reason_type: crate::Lexicon::Com::Atproto::Moderation::Defs::ReasonType,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Did"]
                        pub reported_by: String,
                    }
                    pub enum ActionType {
                        Takedown,
                        Flag,
                        Acknowledge,
                    }
                    #[doc = ""]
                    pub struct ActionViewCurrent {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub action: ActionType,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub id: i64,
                    }
                    #[doc = ""]
                    pub enum ReportViewDetailsubjectType {
                        RepoView(Box<RepoView>),
                        RecordView(Box<RecordView>),
                    }
                    pub struct ReportViewDetail {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub reason_type: crate::Lexicon::Com::Atproto::Moderation::Defs::ReasonType,
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
                        pub reported_by: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Datetime"]
                        pub created_at: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub id: i64,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub reason: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub subject: ReportViewDetailsubjectType,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub resolved_by_actions:
                            Vec<crate::Lexicon::Com::Atproto::Admin::Defs::ActionView>,
                    }
                    #[doc = ""]
                    pub struct Moderation {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub current_action: ActionViewCurrent,
                    }
                    #[doc = ""]
                    pub struct ImageDetails {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub height: i64,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub width: i64,
                    }
                    #[doc = ""]
                    pub struct RecordView {
                        #[doc = "nullable: false"]
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        pub indexed_at: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub moderation: Moderation,
                        #[doc = "format: AtUri"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub uri: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub repo: RepoView,
                        #[doc = "format: Cid"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub cid: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub blob_cids: Vec<String>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub todo4_value: String,
                    }
                    #[doc = ""]
                    pub enum BlobViewdetailsType {
                        ImageDetails(Box<ImageDetails>),
                        VideoDetails(Box<VideoDetails>),
                    }
                    pub struct BlobView {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub moderation: Moderation,
                        #[doc = "required: true"]
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        pub created_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Cid"]
                        pub cid: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub mime_type: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub size: i64,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub details: BlobViewdetailsType,
                    }
                    #[doc = ""]
                    pub struct ActionReversal {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Datetime"]
                        pub created_at: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        pub created_by: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub reason: String,
                    }
                    #[doc = ""]
                    pub enum ActionViewsubjectType {
                        RepoRef(Box<RepoRef>),
                        StrongRefMain,
                    }
                    pub struct ActionView {
                        #[doc = "required: true"]
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        pub created_at: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub id: i64,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub subject: ActionViewsubjectType,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub action: ActionType,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub create_label_vals: Vec<String>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub negate_label_vals: Vec<String>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub reason: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Did"]
                        pub created_by: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub subject_blob_cids: Vec<String>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub resolved_report_ids: Vec<i64>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub reversal: ActionReversal,
                    }
                    #[doc = ""]
                    pub struct RepoRef {
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub did: String,
                    }
                    #[doc = ""]
                    pub struct VideoDetails {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub height: i64,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub width: i64,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub length: i64,
                    }
                    #[doc = ""]
                    pub struct ModerationDetail {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub reports: Vec<ReportView>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub current_action: ActionViewCurrent,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub actions: Vec<ActionView>,
                    }
                    #[doc = ""]
                    pub struct RepoViewDetail {
                        #[doc = "format: Handle"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub handle: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub related_records: Vec<String>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub labels: Vec<crate::Lexicon::Com::Atproto::Label::Defs::Label>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub invited_by: crate::Lexicon::Com::Atproto::Server::Defs::InviteCode,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub invites: Vec<crate::Lexicon::Com::Atproto::Server::Defs::InviteCode>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub moderation: ModerationDetail,
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub indexed_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub email: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        pub did: String,
                    }
                }
                pub mod DisableInviteCodes {
                    #[doc = "Description: \"Disable some set of codes and/or all codes associated with a set of users\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post(
                            "http://localhost:8080/xrpc/com/atproto/admin/disableInviteCodes",
                        );
                        return None;
                    }
                }
                pub mod GetInviteCodes {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub cursor: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub codes: Vec<crate::Lexicon::Com::Atproto::Server::Defs::InviteCode>,
                    }
                    #[doc = "Description: \"Admin view of invite codes\""]
                    pub fn main(cursor: String, sort: String, limit: i64) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/com/atproto/admin/getInviteCodes");
                        return None;
                    }
                }
                pub mod GetModerationAction {
                    #[doc = "Description: \"View details about a moderation action.\""]
                    pub fn main(
                        id: i64,
                    ) -> Option<crate::Lexicon::Com::Atproto::Admin::Defs::ActionViewDetail>
                    {
                        let client = reqwest::blocking::Client::new();
                        client.get(
                            "http://localhost:8080/xrpc/com/atproto/admin/getModerationAction",
                        );
                        return None;
                    }
                }
                pub mod GetModerationActions {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub cursor: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub actions: Vec<crate::Lexicon::Com::Atproto::Admin::Defs::ActionView>,
                    }
                    #[doc = "Description: \"List moderation actions related to a subject.\""]
                    pub fn main(limit: i64, cursor: String, subject: String) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get(
                            "http://localhost:8080/xrpc/com/atproto/admin/getModerationActions",
                        );
                        return None;
                    }
                }
                pub mod GetModerationReport {
                    #[doc = "Description: \"View details about a moderation report.\""]
                    pub fn main(
                        id: i64,
                    ) -> Option<crate::Lexicon::Com::Atproto::Admin::Defs::ReportViewDetail>
                    {
                        let client = reqwest::blocking::Client::new();
                        client.get(
                            "http://localhost:8080/xrpc/com/atproto/admin/getModerationReport",
                        );
                        return None;
                    }
                }
                pub mod GetModerationReports {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub cursor: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub reports: Vec<crate::Lexicon::Com::Atproto::Admin::Defs::ReportView>,
                    }
                    #[doc = "Description: \"List moderation reports related to a subject.\""]
                    pub fn main(
                        resolved: bool,
                        limit: i64,
                        subject: String,
                        cursor: String,
                    ) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get(
                            "http://localhost:8080/xrpc/com/atproto/admin/getModerationReports",
                        );
                        return None;
                    }
                }
                pub mod GetRecord {
                    #[doc = "Description: \"View details about a record.\""]
                    pub fn main(
                        cid: String,
                        uri: String,
                    ) -> Option<crate::Lexicon::Com::Atproto::Admin::Defs::RecordViewDetail>
                    {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/com/atproto/admin/getRecord");
                        return None;
                    }
                }
                pub mod GetRepo {
                    #[doc = "Description: \"View details about a repository.\""]
                    pub fn main(
                        did: String,
                    ) -> Option<crate::Lexicon::Com::Atproto::Admin::Defs::RepoViewDetail>
                    {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/com/atproto/admin/getRepo");
                        return None;
                    }
                }
                pub mod ResolveModerationReports {
                    #[doc = "Description: \"Resolve moderation reports by an action.\""]
                    pub fn main() -> Option<crate::Lexicon::Com::Atproto::Admin::Defs::ActionView> {
                        let client = reqwest::blocking::Client::new();
                        client.post(
                            "http://localhost:8080/xrpc/com/atproto/admin/resolveModerationReports",
                        );
                        return None;
                    }
                }
                pub mod ReverseModerationAction {
                    #[doc = "Description: \"Reverse a moderation action.\""]
                    pub fn main() -> Option<crate::Lexicon::Com::Atproto::Admin::Defs::ActionView> {
                        let client = reqwest::blocking::Client::new();
                        client.post(
                            "http://localhost:8080/xrpc/com/atproto/admin/reverseModerationAction",
                        );
                        return None;
                    }
                }
                pub mod SearchRepos {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub repos: Vec<crate::Lexicon::Com::Atproto::Admin::Defs::RepoView>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub cursor: String,
                    }
                    #[doc = "Description: \"Find repositories based on a search term.\""]
                    pub fn main(
                        term: String,
                        invited_by: String,
                        limit: i64,
                        cursor: String,
                    ) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/com/atproto/admin/searchRepos");
                        return None;
                    }
                }
                pub mod TakeModerationAction {
                    #[doc = "Description: \"Take a moderation action on a repo.\""]
                    pub fn main() -> Option<crate::Lexicon::Com::Atproto::Admin::Defs::ActionView> {
                        let client = reqwest::blocking::Client::new();
                        client.post(
                            "http://localhost:8080/xrpc/com/atproto/admin/takeModerationAction",
                        );
                        return None;
                    }
                }
                pub mod UpdateAccountEmail {
                    #[doc = "Description: \"Administrative action to update an account's email\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post(
                            "http://localhost:8080/xrpc/com/atproto/admin/updateAccountEmail",
                        );
                        return None;
                    }
                }
                pub mod UpdateAccountHandle {
                    #[doc = "Description: \"Administrative action to update an account's handle\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post(
                            "http://localhost:8080/xrpc/com/atproto/admin/updateAccountHandle",
                        );
                        return None;
                    }
                }
            }
            pub mod Identity {
                pub mod ResolveHandle {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Did"]
                        pub did: String,
                    }
                    #[doc = "Description: \"Provides the DID of a repo.\""]
                    pub fn main(handle: String) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/com/atproto/identity/resolveHandle");
                        return None;
                    }
                }
                pub mod UpdateHandle {
                    #[doc = "Description: \"Updates the handle of the account\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/com/atproto/identity/updateHandle");
                        return None;
                    }
                }
            }
            pub mod Label {
                pub mod Defs {
                    #[doc = "Metadata tag on an atproto resource (eg, repo or record)"]
                    pub struct Label {
                        #[doc = "description: \"DID of the actor who created this label\""]
                        #[doc = "required: true"]
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        pub src: String,
                        #[doc = "required: false"]
                        #[doc = "description: \"optionally, CID specifying the specific version of 'uri' resource this label applies to\""]
                        #[doc = "format: Cid"]
                        #[doc = "nullable: false"]
                        pub cid: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Uri"]
                        #[doc = "description: \"AT URI of the record, repository (account), or other resource which this label applies to\""]
                        pub uri: String,
                        #[doc = "required: false"]
                        #[doc = "description: \"if true, this is a negation label, overwriting a previous label\""]
                        #[doc = "nullable: false"]
                        pub neg: bool,
                        #[doc = "description: \"timestamp when this label was created\""]
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub cts: String,
                        #[doc = "required: true"]
                        #[doc = "description: \"the short string name of the value or type of this label\""]
                        #[doc = "nullable: false"]
                        pub val: String,
                    }
                }
                pub mod QueryLabels {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub cursor: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub labels: Vec<crate::Lexicon::Com::Atproto::Label::Defs::Label>,
                    }
                    #[doc = "Description: \"Find labels relevant to the provided URI patterns.\""]
                    pub fn main(
                        cursor: String,
                        uri_patterns: Vec<String>,
                        limit: i64,
                        sources: Vec<String>,
                    ) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/com/atproto/label/queryLabels");
                        return None;
                    }
                }
                pub mod SubscribeLabels {
                    #[doc = ""]
                    pub struct Labels {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub labels: Vec<crate::Lexicon::Com::Atproto::Label::Defs::Label>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub seq: i64,
                    }
                    #[doc = ""]
                    pub enum Name {
                        OutdatedCursor,
                    }
                    pub struct Info {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "known_values: [\"OutdatedCursor\"]"]
                        pub name: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub message: String,
                    }
                }
            }
            pub mod Moderation {
                pub mod CreateReport {
                    #[doc = ""]
                    pub enum MainOutputsubjectType {
                        DefsRepoRef,
                        StrongRefMain,
                    }
                    pub struct MainOutput {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub id: i64,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub subject: MainOutputsubjectType,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub reason: String,
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
                        pub reported_by: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub reason_type: crate::Lexicon::Com::Atproto::Moderation::Defs::ReasonType,
                        #[doc = "nullable: false"]
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        pub created_at: String,
                    }
                    #[doc = "Description: \"Report a repo or a record.\""]
                    pub fn main() -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client
                            .post("http://localhost:8080/xrpc/com/atproto/moderation/createReport");
                        return None;
                    }
                }
                pub mod Defs {
                    pub enum ReasonType {
                        ReasonSpam,
                        ReasonViolation,
                        ReasonMisleading,
                        ReasonSexual,
                        ReasonRude,
                        ReasonOther,
                    }
                }
            }
            pub mod Repo {
                pub mod ApplyWrites {
                    #[doc = "Description: \"Apply a batch transaction of creates, updates, and deletes.\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/com/atproto/repo/applyWrites");
                        return None;
                    }
                    #[doc = "Update an existing record."]
                    pub struct Update {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Nsid"]
                        pub collection: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub rkey: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub todo4_value: String,
                    }
                    #[doc = "Delete an existing record."]
                    pub struct Delete {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub rkey: String,
                        #[doc = "format: Nsid"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub collection: String,
                    }
                    #[doc = "Create a new record."]
                    pub struct Create {
                        #[doc = "nullable: false"]
                        #[doc = "format: Nsid"]
                        #[doc = "required: true"]
                        pub collection: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub rkey: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub todo4_value: String,
                    }
                }
                pub mod CreateRecord {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Cid"]
                        pub cid: String,
                        #[doc = "format: AtUri"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub uri: String,
                    }
                    #[doc = "Description: \"Create a new record.\""]
                    pub fn main() -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/com/atproto/repo/createRecord");
                        return None;
                    }
                }
                pub mod DeleteRecord {
                    #[doc = "Description: \"Delete a record, or ensure it doesn't exist.\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/com/atproto/repo/deleteRecord");
                        return None;
                    }
                }
                pub mod DescribeRepo {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "required: true"]
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        pub did: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub todo4_didDoc: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub handle_is_correct: bool,
                        #[doc = "format: Handle"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub handle: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub collections: Vec<String>,
                    }
                    #[doc = "Description: \"Get information about the repo, including the list of collections.\""]
                    pub fn main(repo: String) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/com/atproto/repo/describeRepo");
                        return None;
                    }
                }
                pub mod GetRecord {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "format: Cid"]
                        #[doc = "required: false"]
                        pub cid: String,
                        #[doc = "required: true"]
                        #[doc = "format: AtUri"]
                        #[doc = "nullable: false"]
                        pub uri: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub todo4_value: String,
                    }
                    #[doc = "Description: \"Get a record.\""]
                    pub fn main(
                        cid: String,
                        collection: String,
                        repo: String,
                        rkey: String,
                    ) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/com/atproto/repo/getRecord");
                        return None;
                    }
                }
                pub mod ListRecords {
                    #[doc = ""]
                    pub struct Record {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Cid"]
                        pub cid: String,
                        #[doc = "format: AtUri"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub uri: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub todo4_value: String,
                    }
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub cursor: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub records: Vec<Record>,
                    }
                    #[doc = "Description: \"List a range of records in a collection.\""]
                    pub fn main(
                        collection: String,
                        reverse: bool,
                        repo: String,
                        rkey_end: String,
                        limit: i64,
                        cursor: String,
                        rkey_start: String,
                    ) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/com/atproto/repo/listRecords");
                        return None;
                    }
                }
                pub mod PutRecord {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "format: Cid"]
                        #[doc = "required: true"]
                        pub cid: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: AtUri"]
                        pub uri: String,
                    }
                    #[doc = "Description: \"Write a record, creating or updating it as needed.\""]
                    pub fn main() -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/com/atproto/repo/putRecord");
                        return None;
                    }
                }
                pub mod StrongRef {
                    #[doc = ""]
                    pub struct Main {
                        #[doc = "format: AtUri"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub uri: String,
                        #[doc = "format: Cid"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub cid: String,
                    }
                }
                pub mod UploadBlob {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub blob: String,
                    }
                    #[doc = "Description: \"Upload a new blob to be added to repo in a later request.\""]
                    pub fn main() -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/com/atproto/repo/uploadBlob");
                        return None;
                    }
                }
            }
            pub mod Server {
                pub mod CreateAccount {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub refresh_jwt: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub access_jwt: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Handle"]
                        pub handle: String,
                        #[doc = "required: true"]
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        pub did: String,
                    }
                    #[doc = "Description: \"Create an account.\""]
                    pub fn main() -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/com/atproto/server/createAccount");
                        return None;
                    }
                }
                pub mod CreateAppPassword {
                    #[doc = ""]
                    pub struct AppPassword {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub name: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub password: String,
                        #[doc = "nullable: false"]
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        pub created_at: String,
                    }
                    #[doc = "Description: \"Create an app-specific password.\""]
                    pub fn main() -> Option<AppPassword> {
                        let client = reqwest::blocking::Client::new();
                        client.post(
                            "http://localhost:8080/xrpc/com/atproto/server/createAppPassword",
                        );
                        return None;
                    }
                }
                pub mod CreateInviteCode {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub code: String,
                    }
                    #[doc = "Description: \"Create an invite code.\""]
                    pub fn main() -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client
                            .post("http://localhost:8080/xrpc/com/atproto/server/createInviteCode");
                        return None;
                    }
                }
                pub mod CreateInviteCodes {
                    #[doc = ""]
                    pub struct AccountCodes {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub account: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub codes: Vec<String>,
                    }
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub codes: Vec<AccountCodes>,
                    }
                    #[doc = "Description: \"Create an invite code.\""]
                    pub fn main() -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.post(
                            "http://localhost:8080/xrpc/com/atproto/server/createInviteCodes",
                        );
                        return None;
                    }
                }
                pub mod CreateSession {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub email: String,
                        #[doc = "format: Handle"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub handle: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub access_jwt: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub refresh_jwt: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Did"]
                        pub did: String,
                    }
                    #[doc = "Description: \"Create an authentication session.\""]
                    pub fn main() -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/com/atproto/server/createSession");
                        return None;
                    }
                }
                pub mod Defs {
                    #[doc = ""]
                    pub struct InviteCodeUse {
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
                        pub used_by: String,
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub used_at: String,
                    }
                    #[doc = ""]
                    pub struct InviteCode {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub disabled: bool,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub code: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub available: i64,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub created_by: String,
                        #[doc = "required: true"]
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        pub created_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub for_account: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub uses: Vec<InviteCodeUse>,
                    }
                }
                pub mod DeleteAccount {
                    #[doc = "Description: \"Delete a user account with a token and password.\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/com/atproto/server/deleteAccount");
                        return None;
                    }
                }
                pub mod DeleteSession {
                    #[doc = "Description: \"Delete the current session.\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/com/atproto/server/deleteSession");
                        return None;
                    }
                }
                pub mod DescribeServer {
                    #[doc = ""]
                    pub struct Links {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub privacy_policy: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub terms_of_service: String,
                    }
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub available_user_domains: Vec<String>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        pub invite_code_required: bool,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub links: Links,
                    }
                    #[doc = "Description: \"Get a document describing the service's accounts configuration.\""]
                    pub fn main() -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/com/atproto/server/describeServer");
                        return None;
                    }
                }
                pub mod GetAccountInviteCodes {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub codes: Vec<crate::Lexicon::Com::Atproto::Server::Defs::InviteCode>,
                    }
                    #[doc = "Description: \"Get all invite codes for a given account\""]
                    pub fn main(include_used: bool, create_available: bool) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get(
                            "http://localhost:8080/xrpc/com/atproto/server/getAccountInviteCodes",
                        );
                        return None;
                    }
                }
                pub mod GetSession {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "format: Handle"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub handle: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        pub did: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub email: String,
                    }
                    #[doc = "Description: \"Get information about the current session.\""]
                    pub fn main() -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/com/atproto/server/getSession");
                        return None;
                    }
                }
                pub mod ListAppPasswords {
                    #[doc = ""]
                    pub struct AppPassword {
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub created_at: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub name: String,
                    }
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub passwords: Vec<AppPassword>,
                    }
                    #[doc = "Description: \"List all app-specific passwords.\""]
                    pub fn main() -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client
                            .get("http://localhost:8080/xrpc/com/atproto/server/listAppPasswords");
                        return None;
                    }
                }
                pub mod RefreshSession {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub refresh_jwt: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Handle"]
                        pub handle: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub access_jwt: String,
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub did: String,
                    }
                    #[doc = "Description: \"Refresh an authentication session.\""]
                    pub fn main() -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/com/atproto/server/refreshSession");
                        return None;
                    }
                }
                pub mod RequestAccountDelete {
                    #[doc = "Description: \"Initiate a user account deletion via email.\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post(
                            "http://localhost:8080/xrpc/com/atproto/server/requestAccountDelete",
                        );
                        return None;
                    }
                }
                pub mod RequestPasswordReset {
                    #[doc = "Description: \"Initiate a user account password reset via email.\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post(
                            "http://localhost:8080/xrpc/com/atproto/server/requestPasswordReset",
                        );
                        return None;
                    }
                }
                pub mod ResetPassword {
                    #[doc = "Description: \"Reset a user account password using a token.\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/com/atproto/server/resetPassword");
                        return None;
                    }
                }
                pub mod RevokeAppPassword {
                    #[doc = "Description: \"Revoke an app-specific password by name.\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post(
                            "http://localhost:8080/xrpc/com/atproto/server/revokeAppPassword",
                        );
                        return None;
                    }
                }
            }
            pub mod Sync {
                pub mod GetBlob {
                    #[doc = "Description: \"Get a blob associated with a given repo.\""]
                    pub fn main(did: String, cid: String) -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/com/atproto/sync/getBlob");
                        return None;
                    }
                }
                pub mod GetBlocks {
                    #[doc = "Description: \"Gets blocks from a given repo.\""]
                    pub fn main(did: String, cids: Vec<String>) -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/com/atproto/sync/getBlocks");
                        return None;
                    }
                }
                pub mod GetCheckout {
                    #[doc = "Description: \"Gets the repo state.\""]
                    pub fn main(commit: String, did: String) -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/com/atproto/sync/getCheckout");
                        return None;
                    }
                }
                pub mod GetCommitPath {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub commits: Vec<String>,
                    }
                    #[doc = "Description: \"Gets the path of repo commits\""]
                    pub fn main(
                        earliest: String,
                        did: String,
                        latest: String,
                    ) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/com/atproto/sync/getCommitPath");
                        return None;
                    }
                }
                pub mod GetHead {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "format: Cid"]
                        #[doc = "required: true"]
                        pub root: String,
                    }
                    #[doc = "Description: \"Gets the current HEAD CID of a repo.\""]
                    pub fn main(did: String) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/com/atproto/sync/getHead");
                        return None;
                    }
                }
                pub mod GetRecord {
                    #[doc = "Description: \"Gets blocks needed for existence or non-existence of record.\""]
                    pub fn main(
                        did: String,
                        collection: String,
                        rkey: String,
                        commit: String,
                    ) -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/com/atproto/sync/getRecord");
                        return None;
                    }
                }
                pub mod GetRepo {
                    #[doc = "Description: \"Gets the repo state.\""]
                    pub fn main(earliest: String, did: String, latest: String) -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/com/atproto/sync/getRepo");
                        return None;
                    }
                }
                pub mod ListBlobs {
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub cids: Vec<String>,
                    }
                    #[doc = "Description: \"List blob cids for some range of commits\""]
                    pub fn main(
                        earliest: String,
                        did: String,
                        latest: String,
                    ) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/com/atproto/sync/listBlobs");
                        return None;
                    }
                }
                pub mod ListRepos {
                    #[doc = ""]
                    pub struct Repo {
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
                        pub did: String,
                        #[doc = "format: Cid"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub head: String,
                    }
                    #[doc = ""]
                    pub struct MainOutput {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub cursor: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub repos: Vec<Repo>,
                    }
                    #[doc = "Description: \"List dids and root cids of hosted repos\""]
                    pub fn main(cursor: String, limit: i64) -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/com/atproto/sync/listRepos");
                        return None;
                    }
                }
                pub mod NotifyOfUpdate {
                    #[doc = "Description: \"Notify a crawling service of a recent update. Often when a long break between updates causes the connection with the crawling service to break.\""]
                    pub fn main(hostname: String) -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/com/atproto/sync/notifyOfUpdate");
                        return None;
                    }
                }
                pub mod RequestCrawl {
                    #[doc = "Description: \"Request a service to persistently crawl hosted repos.\""]
                    pub fn main(hostname: String) -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.get("http://localhost:8080/xrpc/com/atproto/sync/requestCrawl");
                        return None;
                    }
                }
                pub mod SubscribeRepos {
                    #[doc = ""]
                    pub enum Name {
                        OutdatedCursor,
                    }
                    pub struct Info {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        pub message: String,
                        #[doc = "known_values: [\"OutdatedCursor\"]"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub name: String,
                    }
                    #[doc = ""]
                    pub enum Action {
                        Create,
                        Update,
                        Delete,
                    }
                    pub struct RepoOp {
                        #[doc = "nullable: true"]
                        #[doc = "required: true"]
                        pub todo3_cid: Option<String>,
                        #[doc = "nullable: false"]
                        #[doc = "known_values: [\"create\", \"update\", \"delete\"]"]
                        #[doc = "required: true"]
                        pub action: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub path: String,
                    }
                    #[doc = ""]
                    pub struct Commit {
                        #[doc = "required: true"]
                        #[doc = "nullable: true"]
                        pub todo3_prev: Option<String>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub rebase: bool,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub too_big: bool,
                        #[doc = "nullable: false"]
                        #[doc = "description: \"CAR file containing relevant blocks\""]
                        #[doc = "required: true"]
                        pub todo5_blocks: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub todo3_commit: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub ops: Vec<RepoOp>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub blobs: Vec<String>,
                        #[doc = "nullable: false"]
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        pub time: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub seq: i64,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        pub repo: String,
                    }
                    #[doc = ""]
                    pub struct Handle {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Did"]
                        pub did: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Handle"]
                        pub handle: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub seq: i64,
                        #[doc = "nullable: false"]
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        pub time: String,
                    }
                    #[doc = ""]
                    pub struct Migrate {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub seq: i64,
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub time: String,
                        #[doc = "nullable: true"]
                        #[doc = "required: true"]
                        pub migrate_to: Option<String>,
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub did: String,
                    }
                    #[doc = ""]
                    pub struct Tombstone {
                        #[doc = "nullable: false"]
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        pub time: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        pub did: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub seq: i64,
                    }
                }
            }
        }
    }
}
```

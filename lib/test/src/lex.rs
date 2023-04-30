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
                        pub did: String,
                        pub follows_count: i64,
                        pub posts_count: i64,
                        pub handle: String,
                        pub indexed_at: String,
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
                        pub banner: String,
                        pub display_name: String,
                        pub followers_count: i64,
                        pub avatar: String,
                        pub viewer: ViewerState,
                        pub description: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ProfileViewBasic {
                        pub display_name: String,
                        pub did: String,
                        pub viewer: ViewerState,
                        pub handle: String,
                        pub avatar: String,
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ProfileView {
                        pub display_name: String,
                        pub description: String,
                        pub viewer: ViewerState,
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
                        pub avatar: String,
                        pub indexed_at: String,
                        pub handle: String,
                        pub did: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ViewerState {
                        pub blocked_by: bool,
                        pub blocking: String,
                        pub followed_by: String,
                        pub following: String,
                        pub muted: bool,
                    }
                }
                pub mod get_profile {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    pub fn main(
                        token: &String,
                        actor: String,
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
                    pub fn main(
                        token: &String,
                        actors: Vec<String>,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                        term: String,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                    pub fn main(
                        token: &String,
                        limit: i64,
                        term: String,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                    pub struct View {
                        pub external: ViewExternal,
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
                    pub struct External {
                        pub title: String,
                        pub description: String,
                        pub thumb: String,
                        pub uri: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ViewExternal {
                        pub uri: String,
                        pub thumb: String,
                        pub title: String,
                        pub description: String,
                    }
                }
                pub mod images {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct View {
                        pub images: Vec<ViewImage>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Image {
                        pub image: String,
                        pub alt: String,
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
                    pub struct ViewImage {
                        pub fullsize: String,
                        pub alt: String,
                        pub thumb: String,
                    }
                }
                pub mod record {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ViewBlocked {
                        pub uri: String,
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
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum ViewrecordType {
                        ViewRecord(Box<ViewRecord>),
                        ViewNotFound(Box<ViewNotFound>),
                        ViewBlocked(Box<ViewBlocked>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct View {
                        pub record: ViewrecordType,
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
                        pub uri: String,
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
                        pub author: lexicon::app::bsky::actor::defs::ProfileViewBasic,
                        pub todo4_value: String,
                        pub indexed_at: String,
                        pub embeds: Vec<EmbedsType>,
                        pub cid: String,
                    }
                }
                pub mod record_with_media {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum ViewmediaType {
                        ImagesView,
                        ExternalView,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct View {
                        pub media: ViewmediaType,
                        pub record: lexicon::app::bsky::embed::record::View,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum MainmediaType {
                        ImagesMain,
                        ExternalMain,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Main {
                        pub record: lexicon::app::bsky::embed::record::Main,
                        pub media: MainmediaType,
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
                    pub struct NotFoundPost {
                        pub uri: String,
                        pub not_found: bool,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum ThreadViewPostparentType {
                        ThreadViewPost(Box<ThreadViewPost>),
                        NotFoundPost(Box<NotFoundPost>),
                        BlockedPost(Box<BlockedPost>),
                    }
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
                    pub struct ThreadViewPost {
                        pub post: PostView,
                        pub parent: ThreadViewPostparentType,
                        pub replies: Vec<RepliesType>,
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
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum PostViewembedType {
                        ImagesView,
                        ExternalView,
                        RecordView,
                        RecordWithMediaView,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct PostView {
                        pub embed: PostViewembedType,
                        pub repost_count: i64,
                        pub uri: String,
                        pub cid: String,
                        pub author: lexicon::app::bsky::actor::defs::ProfileViewBasic,
                        pub todo4_record: String,
                        pub like_count: i64,
                        pub indexed_at: String,
                        pub reply_count: i64,
                        pub viewer: ViewerState,
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
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
                    pub enum FeedViewPostreasonType {
                        ReasonRepost(Box<ReasonRepost>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct FeedViewPost {
                        pub reply: ReplyRef,
                        pub reason: FeedViewPostreasonType,
                        pub post: lexicon::app::bsky::feed::defs::PostView,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ReasonRepost {
                        pub indexed_at: String,
                        pub by: lexicon::app::bsky::actor::defs::ProfileViewBasic,
                    }
                }
                pub mod get_author_feed {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub feed: Vec<lexicon::app::bsky::feed::defs::FeedViewPost>,
                        pub cursor: String,
                    }
                    #[doc = "Description: \"A view of an actor's feed.\""]
                    pub fn main(
                        token: &String,
                        actor: String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                    pub struct MainOutput {
                        pub likes: Vec<Like>,
                        pub cid: String,
                        pub uri: String,
                        pub cursor: String,
                    }
                    pub fn main(
                        token: &String,
                        cid: String,
                        cursor: String,
                        limit: i64,
                        uri: String,
                    ) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/app.bsky.feed.getLikes")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Like {
                        pub actor: lexicon::app::bsky::actor::defs::ProfileView,
                        pub indexed_at: String,
                        pub created_at: String,
                    }
                }
                pub mod get_post_thread {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum MainOutputthreadType {
                        DefsThreadViewPost,
                        DefsNotFoundPost,
                        DefsBlockedPost,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub thread: MainOutputthreadType,
                    }
                    pub fn main(
                        token: &String,
                        depth: i64,
                        uri: String,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                    pub fn main(
                        token: &String,
                        uris: Vec<String>,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                        pub reposted_by: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                        pub cid: String,
                        pub cursor: String,
                        pub uri: String,
                    }
                    pub fn main(
                        token: &String,
                        cid: String,
                        cursor: String,
                        limit: i64,
                        uri: String,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                    pub fn main(
                        token: &String,
                        algorithm: String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        let resp= client
                            .get("http://bsky.social/xrpc/app.bsky.feed.getTimeline")
                            .header("accept", "application/json")
                            .header("Authorization", token)
                            .send()?;
                        println!("res {:?}", resp.text()?);
                        panic!();
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
                        pub index: TextSlice,
                        pub value: String,
                        pub r#type: String,
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
                        pub start: i64,
                        pub end: i64,
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
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                    pub fn main(
                        token: &String,
                        actor: String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                        pub cursor: String,
                        pub subject: lexicon::app::bsky::actor::defs::ProfileView,
                        pub follows: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                    }
                    #[doc = "Description: \"Who is an actor following?\""]
                    pub fn main(
                        token: &String,
                        actor: String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/app/bsky/graph/muteActor");
                        return None;
                    }
                }
                pub mod unmute_actor {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Unmute an actor by did or handle.\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/app/bsky/graph/unmuteActor");
                        return None;
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
                    pub fn main(
                        token: &String,
                        seen_at: String,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                    pub enum Reason {
                        Like,
                        Repost,
                        Follow,
                        Mention,
                        Reply,
                        Quote,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Notification {
                        pub todo4_record: String,
                        pub reason: String,
                        pub reason_subject: String,
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
                        pub cid: String,
                        pub uri: String,
                        pub is_read: bool,
                        pub author: lexicon::app::bsky::actor::defs::ProfileView,
                        pub indexed_at: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub cursor: String,
                        pub notifications: Vec<Notification>,
                    }
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                        seen_at: String,
                    ) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/app.bsky.notification.listNotifications")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod update_seen {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Notify server that the user has seen notifications.\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/app/bsky/notification/updateSeen");
                        return None;
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
                    pub enum FeaturesType {
                        Mention(Box<Mention>),
                        Link(Box<Link>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Main {
                        pub index: ByteSlice,
                        pub features: Vec<FeaturesType>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Link {
                        pub uri: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ByteSlice {
                        pub byte_end: i64,
                        pub byte_start: i64,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Mention {
                        pub did: String,
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
                        pub feed: Vec<lexicon::app::bsky::feed::defs::FeedViewPost>,
                        pub cursor: String,
                    }
                    #[doc = "Description: \"An unspecced view of globally popular items\""]
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                    pub enum ActionType {
                        Takedown,
                        Flag,
                        Acknowledge,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum ReportViewsubjectType {
                        RepoRef(Box<RepoRef>),
                        StrongRefMain,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ReportView {
                        pub id: i64,
                        pub created_at: String,
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                        pub subject: ReportViewsubjectType,
                        pub reported_by: String,
                        pub reason: String,
                        pub resolved_by_action_ids: Vec<i64>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ActionReversal {
                        pub created_at: String,
                        pub reason: String,
                        pub created_by: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct RecordViewDetail {
                        pub todo4_value: String,
                        pub cid: String,
                        pub blobs: Vec<BlobView>,
                        pub repo: RepoView,
                        pub indexed_at: String,
                        pub uri: String,
                        pub moderation: ModerationDetail,
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
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
                    pub enum BlobViewdetailsType {
                        ImageDetails(Box<ImageDetails>),
                        VideoDetails(Box<VideoDetails>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct BlobView {
                        pub cid: String,
                        pub size: i64,
                        pub moderation: Moderation,
                        pub details: BlobViewdetailsType,
                        pub created_at: String,
                        pub mime_type: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct VideoDetails {
                        pub length: i64,
                        pub height: i64,
                        pub width: i64,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum ActionViewsubjectType {
                        RepoRef(Box<RepoRef>),
                        StrongRefMain,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ActionView {
                        pub negate_label_vals: Vec<String>,
                        pub created_by: String,
                        pub action: ActionType,
                        pub created_at: String,
                        pub resolved_report_ids: Vec<i64>,
                        pub reason: String,
                        pub reversal: ActionReversal,
                        pub id: i64,
                        pub subject: ActionViewsubjectType,
                        pub subject_blob_cids: Vec<String>,
                        pub create_label_vals: Vec<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum ActionViewDetailsubjectType {
                        RepoView(Box<RepoView>),
                        RecordView(Box<RecordView>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ActionViewDetail {
                        pub action: ActionType,
                        pub created_at: String,
                        pub resolved_reports: Vec<ReportView>,
                        pub id: i64,
                        pub subject: ActionViewDetailsubjectType,
                        pub create_label_vals: Vec<String>,
                        pub reason: String,
                        pub reversal: ActionReversal,
                        pub created_by: String,
                        pub negate_label_vals: Vec<String>,
                        pub subject_blobs: Vec<BlobView>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct RepoView {
                        pub did: String,
                        pub related_records: Vec<String>,
                        pub handle: String,
                        pub moderation: Moderation,
                        pub invited_by: lexicon::com::atproto::server::defs::InviteCode,
                        pub email: String,
                        pub indexed_at: String,
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
                    pub struct ActionViewCurrent {
                        pub id: i64,
                        pub action: ActionType,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ModerationDetail {
                        pub reports: Vec<ReportView>,
                        pub current_action: ActionViewCurrent,
                        pub actions: Vec<ActionView>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct RecordView {
                        pub cid: String,
                        pub blob_cids: Vec<String>,
                        pub repo: RepoView,
                        pub todo4_value: String,
                        pub indexed_at: String,
                        pub moderation: Moderation,
                        pub uri: String,
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
                    pub enum ReportViewDetailsubjectType {
                        RepoView(Box<RepoView>),
                        RecordView(Box<RecordView>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct ReportViewDetail {
                        pub subject: ReportViewDetailsubjectType,
                        pub created_at: String,
                        pub reported_by: String,
                        pub resolved_by_actions:
                            Vec<lexicon::com::atproto::admin::defs::ActionView>,
                        pub reason: String,
                        pub id: i64,
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct RepoViewDetail {
                        pub email: String,
                        pub handle: String,
                        pub invites: Vec<lexicon::com::atproto::server::defs::InviteCode>,
                        pub moderation: ModerationDetail,
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
                        pub invited_by: lexicon::com::atproto::server::defs::InviteCode,
                        pub did: String,
                        pub related_records: Vec<String>,
                        pub indexed_at: String,
                    }
                }
                pub mod disable_invite_codes {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Disable some set of codes and/or all codes associated with a set of users\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post(
                            "http://localhost:8080/xrpc/com/atproto/admin/disableInviteCodes",
                        );
                        return None;
                    }
                }
                pub mod get_invite_codes {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub cursor: String,
                        pub codes: Vec<lexicon::com::atproto::server::defs::InviteCode>,
                    }
                    #[doc = "Description: \"Admin view of invite codes\""]
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                        sort: String,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                        id: i64,
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
                        pub cursor: String,
                        pub actions: Vec<lexicon::com::atproto::admin::defs::ActionView>,
                    }
                    #[doc = "Description: \"List moderation actions related to a subject.\""]
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                        subject: String,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                        id: i64,
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
                        pub cursor: String,
                        pub reports: Vec<lexicon::com::atproto::admin::defs::ReportView>,
                    }
                    #[doc = "Description: \"List moderation reports related to a subject.\""]
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                        resolved: bool,
                        subject: String,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                        cid: String,
                        uri: String,
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
                        did: String,
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
                    pub fn main() -> Option<lexicon::com::atproto::admin::defs::ActionView> {
                        let client = reqwest::blocking::Client::new();
                        client.post(
                            "http://localhost:8080/xrpc/com/atproto/admin/resolveModerationReports",
                        );
                        return None;
                    }
                }
                pub mod reverse_moderation_action {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Reverse a moderation action.\""]
                    pub fn main() -> Option<lexicon::com::atproto::admin::defs::ActionView> {
                        let client = reqwest::blocking::Client::new();
                        client.post(
                            "http://localhost:8080/xrpc/com/atproto/admin/reverseModerationAction",
                        );
                        return None;
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
                    pub fn main(
                        token: &String,
                        cursor: String,
                        invited_by: String,
                        limit: i64,
                        term: String,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                    pub fn main() -> Option<lexicon::com::atproto::admin::defs::ActionView> {
                        let client = reqwest::blocking::Client::new();
                        client.post(
                            "http://localhost:8080/xrpc/com/atproto/admin/takeModerationAction",
                        );
                        return None;
                    }
                }
                pub mod update_account_email {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Administrative action to update an account's email\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post(
                            "http://localhost:8080/xrpc/com/atproto/admin/updateAccountEmail",
                        );
                        return None;
                    }
                }
                pub mod update_account_handle {
                    #[allow(unused_imports)]
                    use super::lexicon;
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
                    pub fn main(
                        token: &String,
                        handle: String,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/com/atproto/identity/updateHandle");
                        return None;
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
                        pub cts: String,
                        pub src: String,
                        pub uri: String,
                        pub cid: String,
                        pub val: String,
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
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                        sources: Vec<String>,
                        uri_patterns: Vec<String>,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                    pub enum Name {
                        OutdatedCursor,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Info {
                        pub name: String,
                        pub message: String,
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
                    pub enum MainOutputsubjectType {
                        DefsRepoRef,
                        StrongRefMain,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                        pub created_at: String,
                        pub id: i64,
                        pub reported_by: String,
                        pub reason: String,
                        pub subject: MainOutputsubjectType,
                    }
                    #[doc = "Description: \"Report a repo or a record.\""]
                    pub fn main() -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client
                            .post("http://localhost:8080/xrpc/com/atproto/moderation/createReport");
                        return None;
                    }
                }
                pub mod defs {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
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
            pub mod repo {
                #[allow(unused_imports)]
                use super::lexicon;
                pub mod apply_writes {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Delete {
                        pub collection: String,
                        pub rkey: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Update {
                        pub collection: String,
                        pub rkey: String,
                        pub todo4_value: String,
                    }
                    #[doc = "Description: \"Apply a batch transaction of creates, updates, and deletes.\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/com/atproto/repo/applyWrites");
                        return None;
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Create {
                        pub todo4_value: String,
                        pub rkey: String,
                        pub collection: String,
                    }
                }
                pub mod create_record {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub uri: String,
                        pub cid: String,
                    }
                    #[doc = "Description: \"Create a new record.\""]
                    pub fn main() -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/com/atproto/repo/createRecord");
                        return None;
                    }
                }
                pub mod delete_record {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Delete a record, or ensure it doesn't exist.\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/com/atproto/repo/deleteRecord");
                        return None;
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
                        pub todo4_didDoc: String,
                        pub collections: Vec<String>,
                        pub did: String,
                        pub handle_is_correct: bool,
                    }
                    #[doc = "Description: \"Get information about the repo, including the list of collections.\""]
                    pub fn main(
                        token: &String,
                        repo: String,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                        pub uri: String,
                        pub cid: String,
                        pub todo4_value: String,
                    }
                    #[doc = "Description: \"Get a record.\""]
                    pub fn main(
                        token: &String,
                        cid: String,
                        collection: String,
                        repo: String,
                        rkey: String,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                    pub struct MainOutput {
                        pub cursor: String,
                        pub records: Vec<Record>,
                    }
                    #[doc = "Description: \"List a range of records in a collection.\""]
                    pub fn main(
                        token: &String,
                        collection: String,
                        cursor: String,
                        limit: i64,
                        repo: String,
                        reverse: bool,
                        rkey_end: String,
                        rkey_start: String,
                    ) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.repo.listRecords")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Record {
                        pub uri: String,
                        pub cid: String,
                        pub todo4_value: String,
                    }
                }
                pub mod put_record {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub cid: String,
                        pub uri: String,
                    }
                    #[doc = "Description: \"Write a record, creating or updating it as needed.\""]
                    pub fn main() -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/com/atproto/repo/putRecord");
                        return None;
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
                    pub struct MainOutput {
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
            pub mod server {
                #[allow(unused_imports)]
                use super::lexicon;
                pub mod create_account {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub access_jwt: String,
                        pub refresh_jwt: String,
                        pub handle: String,
                        pub did: String,
                    }
                    #[doc = "Description: \"Create an account.\""]
                    pub fn main() -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/com/atproto/server/createAccount");
                        return None;
                    }
                }
                pub mod create_app_password {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct AppPassword {
                        pub password: String,
                        pub created_at: String,
                        pub name: String,
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
                pub mod create_invite_code {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
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
                pub mod create_invite_codes {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct AccountCodes {
                        pub account: String,
                        pub codes: Vec<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
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
                pub mod create_session {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub access_jwt: String,
                        pub handle: String,
                        pub email: String,
                        pub refresh_jwt: String,
                        pub did: String,
                    }
                    #[doc = "Description: \"Create an authentication session.\""]
                    pub fn main() -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/com/atproto/server/createSession");
                        return None;
                    }
                }
                pub mod defs {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct InviteCode {
                        pub uses: Vec<InviteCodeUse>,
                        pub disabled: bool,
                        pub code: String,
                        pub created_at: String,
                        pub available: i64,
                        pub for_account: String,
                        pub created_by: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct InviteCodeUse {
                        pub used_by: String,
                        pub used_at: String,
                    }
                }
                pub mod delete_account {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Delete a user account with a token and password.\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/com/atproto/server/deleteAccount");
                        return None;
                    }
                }
                pub mod delete_session {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Delete the current session.\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/com/atproto/server/deleteSession");
                        return None;
                    }
                }
                pub mod describe_server {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Links {
                        pub privacy_policy: String,
                        pub terms_of_service: String,
                    }
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
                    pub fn main(
                        token: &String,
                        create_available: bool,
                        include_used: bool,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                        pub did: String,
                        pub handle: String,
                        pub email: String,
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
                    pub struct AppPassword {
                        pub created_at: String,
                        pub name: String,
                    }
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
                }
                pub mod refresh_session {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct MainOutput {
                        pub access_jwt: String,
                        pub handle: String,
                        pub did: String,
                        pub refresh_jwt: String,
                    }
                    #[doc = "Description: \"Refresh an authentication session.\""]
                    pub fn main() -> Option<MainOutput> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/com/atproto/server/refreshSession");
                        return None;
                    }
                }
                pub mod request_account_delete {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Initiate a user account deletion via email.\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post(
                            "http://localhost:8080/xrpc/com/atproto/server/requestAccountDelete",
                        );
                        return None;
                    }
                }
                pub mod request_password_reset {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Initiate a user account password reset via email.\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post(
                            "http://localhost:8080/xrpc/com/atproto/server/requestPasswordReset",
                        );
                        return None;
                    }
                }
                pub mod reset_password {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Reset a user account password using a token.\""]
                    pub fn main() -> Option<()> {
                        let client = reqwest::blocking::Client::new();
                        client.post("http://localhost:8080/xrpc/com/atproto/server/resetPassword");
                        return None;
                    }
                }
                pub mod revoke_app_password {
                    #[allow(unused_imports)]
                    use super::lexicon;
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
            pub mod sync {
                #[allow(unused_imports)]
                use super::lexicon;
                pub mod get_blob {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Get a blob associated with a given repo.\""]
                    pub fn main(
                        token: &String,
                        cid: String,
                        did: String,
                    ) -> Result<(), reqwest::Error> {
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
                    pub fn main(
                        token: &String,
                        cids: Vec<String>,
                        did: String,
                    ) -> Result<(), reqwest::Error> {
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
                    pub fn main(
                        token: &String,
                        commit: String,
                        did: String,
                    ) -> Result<(), reqwest::Error> {
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
                    pub fn main(
                        token: &String,
                        did: String,
                        earliest: String,
                        latest: String,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                    pub fn main(token: &String, did: String) -> Result<MainOutput, reqwest::Error> {
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
                    pub fn main(
                        token: &String,
                        collection: String,
                        commit: String,
                        did: String,
                        rkey: String,
                    ) -> Result<(), reqwest::Error> {
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
                    pub fn main(
                        token: &String,
                        did: String,
                        earliest: String,
                        latest: String,
                    ) -> Result<(), reqwest::Error> {
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
                    pub fn main(
                        token: &String,
                        did: String,
                        earliest: String,
                        latest: String,
                    ) -> Result<MainOutput, reqwest::Error> {
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
                    pub struct MainOutput {
                        pub repos: Vec<Repo>,
                        pub cursor: String,
                    }
                    #[doc = "Description: \"List dids and root cids of hosted repos\""]
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("http://bsky.social/xrpc/com.atproto.sync.listRepos")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Repo {
                        pub head: String,
                        pub did: String,
                    }
                }
                pub mod notify_of_update {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[doc = "Description: \"Notify a crawling service of a recent update. Often when a long break between updates causes the connection with the crawling service to break.\""]
                    pub fn main(token: &String, hostname: String) -> Result<(), reqwest::Error> {
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
                    pub fn main(token: &String, hostname: String) -> Result<(), reqwest::Error> {
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
                    pub enum Name {
                        OutdatedCursor,
                    }
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
                    pub struct Migrate {
                        pub time: String,
                        pub seq: i64,
                        pub did: String,
                        pub migrate_to: Option<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Commit {
                        pub rebase: bool,
                        pub seq: i64,
                        pub todo4_prev: Option<String>,
                        pub blobs: Vec<String>,
                        pub time: String,
                        pub todo5_blocks: String,
                        pub ops: Vec<RepoOp>,
                        pub todo4_commit: String,
                        pub too_big: bool,
                        pub repo: String,
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
                    pub enum Action {
                        Create,
                        Update,
                        Delete,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct RepoOp {
                        pub action: String,
                        pub todo4_cid: Option<String>,
                        pub path: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Handle {
                        pub time: String,
                        pub seq: i64,
                        pub did: String,
                        pub handle: String,
                    }
                }
            }
        }
    }
}

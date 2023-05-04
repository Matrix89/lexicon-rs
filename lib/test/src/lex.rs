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
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ProfileViewBasic {
                        pub display_name: Option<String>,
                        pub did: String,
                        pub handle: String,
                        pub viewer: Option<ViewerState>,
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        pub avatar: Option<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ProfileView {
                        pub handle: String,
                        pub avatar: Option<String>,
                        pub display_name: Option<String>,
                        pub viewer: Option<ViewerState>,
                        pub did: String,
                        pub description: Option<String>,
                        pub indexed_at: Option<String>,
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ViewerState {
                        pub following: Option<String>,
                        pub blocking: Option<String>,
                        pub muted: Option<bool>,
                        pub blocked_by: Option<bool>,
                        pub followed_by: Option<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ProfileViewDetailed {
                        pub banner: Option<String>,
                        pub handle: String,
                        pub did: String,
                        pub followers_count: Option<i64>,
                        pub follows_count: Option<i64>,
                        pub avatar: Option<String>,
                        pub display_name: Option<String>,
                        pub viewer: Option<ViewerState>,
                        pub indexed_at: Option<String>,
                        pub description: Option<String>,
                        pub posts_count: Option<i64>,
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                    }
                }
                pub mod get_profile {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    pub fn main(
                        token: &String,
                        actor: String,
                    ) -> Result<lexicon::app::bsky::actor::defs::ProfileViewDetailed, reqwest::Error>
                    {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("https://bsky.social/xrpc/app.bsky.actor.getProfile")
                            .header("Authorization", token)
                            .send()?
                            .json::<lexicon::app::bsky::actor::defs::ProfileViewDetailed>();
                    }
                }
                pub mod get_profiles {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub profiles: Vec<lexicon::app::bsky::actor::defs::ProfileViewDetailed>,
                    }
                    pub fn main(
                        token: &String,
                        actors: Vec<String>,
                    ) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("https://bsky.social/xrpc/app.bsky.actor.getProfiles")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_suggestions {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub cursor: Option<String>,
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
                            .get("https://bsky.social/xrpc/app.bsky.actor.getSuggestions")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod profile {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                }
                pub mod search_actors {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub actors: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                        pub cursor: Option<String>,
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
                            .get("https://bsky.social/xrpc/app.bsky.actor.searchActors")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod search_actors_typeahead {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
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
                            .get("https://bsky.social/xrpc/app.bsky.actor.searchActorsTypeahead")
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
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Main {
                        pub external: External,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ViewExternal {
                        pub title: String,
                        pub description: String,
                        pub uri: String,
                        pub thumb: Option<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct View {
                        pub external: ViewExternal,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct External {
                        pub thumb: Option<String>,
                        pub title: String,
                        pub description: String,
                        pub uri: String,
                    }
                }
                pub mod images {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Main {
                        pub images: Vec<Image>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Image {
                        pub alt: String,
                        pub image: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct View {
                        pub images: Vec<ViewImage>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ViewImage {
                        pub alt: String,
                        pub thumb: String,
                        pub fullsize: String,
                    }
                }
                pub mod record {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
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
                    #[serde(rename_all = "camelCase")]
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
                    #[serde(rename_all = "camelCase")]
                    pub struct ViewRecord {
                        pub author: lexicon::app::bsky::actor::defs::ProfileViewBasic,
                        pub embeds: Option<Vec<EmbedsType>>,
                        pub indexed_at: String,
                        pub value: String,
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        pub uri: String,
                        pub cid: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Main {
                        pub record: lexicon::com::atproto::repo::strong_ref::Main,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ViewNotFound {
                        pub uri: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ViewBlocked {
                        pub uri: String,
                    }
                }
                pub mod record_with_media {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
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
                    #[serde(rename_all = "camelCase")]
                    pub struct Main {
                        pub record: lexicon::app::bsky::embed::record::Main,
                        pub media: MainmediaType,
                    }
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
                    #[serde(rename_all = "camelCase")]
                    pub struct View {
                        pub record: lexicon::app::bsky::embed::record::View,
                        pub media: ViewmediaType,
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
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
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
                    #[serde(rename_all = "camelCase")]
                    pub struct PostView {
                        pub repost_count: Option<i64>,
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        pub embed: Option<PostViewembedType>,
                        pub uri: String,
                        pub author: lexicon::app::bsky::actor::defs::ProfileViewBasic,
                        pub like_count: Option<i64>,
                        pub indexed_at: String,
                        pub cid: String,
                        pub record: String,
                        pub viewer: Option<ViewerState>,
                        pub reply_count: Option<i64>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ReasonRepost {
                        pub by: lexicon::app::bsky::actor::defs::ProfileViewBasic,
                        pub indexed_at: String,
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
                    #[serde(rename_all = "camelCase")]
                    pub struct FeedViewPost {
                        pub post: lexicon::app::bsky::feed::defs::PostView,
                        pub reply: Option<ReplyRef>,
                        pub reason: Option<FeedViewPostreasonType>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct BlockedPost {
                        pub blocked: bool,
                        pub uri: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct NotFoundPost {
                        pub uri: String,
                        pub not_found: bool,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ReplyRef {
                        pub root: lexicon::app::bsky::feed::defs::PostView,
                        pub parent: lexicon::app::bsky::feed::defs::PostView,
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
                    pub enum ThreadViewPostparentType {
                        ThreadViewPost(Box<ThreadViewPost>),
                        NotFoundPost(Box<NotFoundPost>),
                        BlockedPost(Box<BlockedPost>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ThreadViewPost {
                        pub post: PostView,
                        pub replies: Option<Vec<RepliesType>>,
                        pub parent: Option<ThreadViewPostparentType>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ViewerState {
                        pub repost: Option<String>,
                        pub like: Option<String>,
                    }
                }
                pub mod get_author_feed {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub feed: Vec<lexicon::app::bsky::feed::defs::FeedViewPost>,
                        pub cursor: Option<String>,
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
                            .get("https://bsky.social/xrpc/app.bsky.feed.getAuthorFeed")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_likes {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub cid: Option<String>,
                        pub uri: String,
                        pub cursor: Option<String>,
                        pub likes: Vec<Like>,
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
                            .get("https://bsky.social/xrpc/app.bsky.feed.getLikes")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Like {
                        pub indexed_at: String,
                        pub created_at: String,
                        pub actor: lexicon::app::bsky::actor::defs::ProfileView,
                    }
                }
                pub mod get_post_thread {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
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
                    #[serde(rename_all = "camelCase")]
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
                            .get("https://bsky.social/xrpc/app.bsky.feed.getPostThread")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_posts {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
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
                            .get("https://bsky.social/xrpc/app.bsky.feed.getPosts")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_reposted_by {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub cursor: Option<String>,
                        pub uri: String,
                        pub reposted_by: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                        pub cid: Option<String>,
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
                            .get("https://bsky.social/xrpc/app.bsky.feed.getRepostedBy")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_timeline {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub cursor: Option<String>,
                        pub feed: Vec<lexicon::app::bsky::feed::defs::FeedViewPost>,
                    }
                    #[doc = "Description: \"A view of the user's home timeline.\""]
                    pub fn main(
                        token: &String,
                        algorithm: String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("https://bsky.social/xrpc/app.bsky.feed.getTimeline")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod like {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                }
                pub mod post {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ReplyRef {
                        pub parent: lexicon::com::atproto::repo::strong_ref::Main,
                        pub root: lexicon::com::atproto::repo::strong_ref::Main,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Entity {
                        pub value: String,
                        pub index: TextSlice,
                        pub r#type: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct TextSlice {
                        pub start: i64,
                        pub end: i64,
                    }
                }
                pub mod repost {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                }
            }
            pub mod graph {
                #[allow(unused_imports)]
                use super::lexicon;
                pub mod block {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                }
                pub mod follow {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                }
                pub mod get_blocks {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub blocks: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                        pub cursor: Option<String>,
                    }
                    #[doc = "Description: \"Who is the requester's account blocking?\""]
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("https://bsky.social/xrpc/app.bsky.graph.getBlocks")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_followers {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub subject: lexicon::app::bsky::actor::defs::ProfileView,
                        pub followers: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                        pub cursor: Option<String>,
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
                            .get("https://bsky.social/xrpc/app.bsky.graph.getFollowers")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_follows {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub subject: lexicon::app::bsky::actor::defs::ProfileView,
                        pub cursor: Option<String>,
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
                            .get("https://bsky.social/xrpc/app.bsky.graph.getFollows")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_mutes {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub mutes: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                        pub cursor: Option<String>,
                    }
                    #[doc = "Description: \"Who does the viewer mute?\""]
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("https://bsky.social/xrpc/app.bsky.graph.getMutes")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod mute_actor {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub actor: String,
                    }
                    #[doc = "Description: \"Mute an actor by did or handle.\""]
                    pub fn main(token: &String, input: MainInput) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("https://bsky.social/xrpc/app.bsky.graph.muteActor")
                            .header("Authorization", token)
                            .json(&input)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod unmute_actor {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub actor: String,
                    }
                    #[doc = "Description: \"Unmute an actor by did or handle.\""]
                    pub fn main(token: &String, input: MainInput) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("https://bsky.social/xrpc/app.bsky.graph.unmuteActor")
                            .header("Authorization", token)
                            .json(&input)
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
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub count: i64,
                    }
                    pub fn main(
                        token: &String,
                        seen_at: String,
                    ) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("https://bsky.social/xrpc/app.bsky.notification.getUnreadCount")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod list_notifications {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub notifications: Vec<Notification>,
                        pub cursor: Option<String>,
                    }
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                        seen_at: String,
                    ) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("https://bsky.social/xrpc/app.bsky.notification.listNotifications")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Notification {
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        pub is_read: bool,
                        pub indexed_at: String,
                        pub cid: String,
                        pub record: String,
                        pub reason_subject: Option<String>,
                        pub reason: String,
                        pub uri: String,
                        pub author: lexicon::app::bsky::actor::defs::ProfileView,
                    }
                }
                pub mod update_seen {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub seen_at: String,
                    }
                    #[doc = "Description: \"Notify server that the user has seen notifications.\""]
                    pub fn main(token: &String, input: MainInput) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("https://bsky.social/xrpc/app.bsky.notification.updateSeen")
                            .header("Authorization", token)
                            .json(&input)
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
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Mention {
                        pub did: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Link {
                        pub uri: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ByteSlice {
                        pub byte_end: i64,
                        pub byte_start: i64,
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
                    #[serde(rename_all = "camelCase")]
                    pub struct Main {
                        pub index: ByteSlice,
                        pub features: Vec<FeaturesType>,
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
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub feed: Vec<lexicon::app::bsky::feed::defs::FeedViewPost>,
                        pub cursor: Option<String>,
                    }
                    #[doc = "Description: \"An unspecced view of globally popular items\""]
                    pub fn main(
                        token: &String,
                        cursor: String,
                        include_nsfw: bool,
                        limit: i64,
                    ) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("https://bsky.social/xrpc/app.bsky.unspecced.getPopular")
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
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ActionReversal {
                        pub reason: String,
                        pub created_at: String,
                        pub created_by: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RepoRef {
                        pub did: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ActionViewCurrent {
                        pub action: ActionType,
                        pub id: i64,
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
                    #[serde(rename_all = "camelCase")]
                    pub struct ReportViewDetail {
                        pub id: i64,
                        pub subject: ReportViewDetailsubjectType,
                        pub created_at: String,
                        pub resolved_by_actions:
                            Vec<lexicon::com::atproto::admin::defs::ActionView>,
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                        pub reason: Option<String>,
                        pub reported_by: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RepoView {
                        pub did: String,
                        pub indexed_at: String,
                        pub email: Option<String>,
                        pub handle: String,
                        pub related_records: Vec<Unimplemented>,
                        pub moderation: Moderation,
                        pub invited_by: Option<lexicon::com::atproto::server::defs::InviteCode>,
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
                    #[serde(rename_all = "camelCase")]
                    pub struct BlobView {
                        pub created_at: String,
                        pub mime_type: String,
                        pub cid: String,
                        pub size: i64,
                        pub details: Option<BlobViewdetailsType>,
                        pub moderation: Option<Moderation>,
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
                    #[serde(rename_all = "camelCase")]
                    pub struct ActionView {
                        pub subject: ActionViewsubjectType,
                        pub action: ActionType,
                        pub create_label_vals: Option<Vec<String>>,
                        pub reason: String,
                        pub created_by: String,
                        pub id: i64,
                        pub negate_label_vals: Option<Vec<String>>,
                        pub created_at: String,
                        pub reversal: Option<ActionReversal>,
                        pub resolved_report_ids: Vec<i64>,
                        pub subject_blob_cids: Vec<String>,
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
                    #[serde(rename_all = "camelCase")]
                    pub struct ActionViewDetail {
                        pub resolved_reports: Vec<ReportView>,
                        pub created_at: String,
                        pub reason: String,
                        pub reversal: Option<ActionReversal>,
                        pub negate_label_vals: Option<Vec<String>>,
                        pub id: i64,
                        pub subject: ActionViewDetailsubjectType,
                        pub create_label_vals: Option<Vec<String>>,
                        pub created_by: String,
                        pub action: ActionType,
                        pub subject_blobs: Vec<BlobView>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ModerationDetail {
                        pub actions: Vec<ActionView>,
                        pub reports: Vec<ReportView>,
                        pub current_action: Option<ActionViewCurrent>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RecordView {
                        pub repo: RepoView,
                        pub cid: String,
                        pub blob_cids: Vec<String>,
                        pub indexed_at: String,
                        pub value: String,
                        pub uri: String,
                        pub moderation: Moderation,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum ActionType {}
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RecordViewDetail {
                        pub moderation: ModerationDetail,
                        pub repo: RepoView,
                        pub blobs: Vec<BlobView>,
                        pub cid: String,
                        pub value: String,
                        pub indexed_at: String,
                        pub uri: String,
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Moderation {
                        pub current_action: Option<ActionViewCurrent>,
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
                    #[serde(rename_all = "camelCase")]
                    pub struct ReportView {
                        pub created_at: String,
                        pub resolved_by_action_ids: Vec<i64>,
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                        pub reported_by: String,
                        pub reason: Option<String>,
                        pub id: i64,
                        pub subject: ReportViewsubjectType,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RepoViewDetail {
                        pub handle: String,
                        pub related_records: Vec<Unimplemented>,
                        pub moderation: ModerationDetail,
                        pub email: Option<String>,
                        pub indexed_at: String,
                        pub did: String,
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        pub invited_by: Option<lexicon::com::atproto::server::defs::InviteCode>,
                        pub invites: Option<Vec<lexicon::com::atproto::server::defs::InviteCode>>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ImageDetails {
                        pub height: i64,
                        pub width: i64,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct VideoDetails {
                        pub length: i64,
                        pub width: i64,
                        pub height: i64,
                    }
                }
                pub mod disable_invite_codes {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub accounts: Option<Vec<String>>,
                        pub codes: Option<Vec<String>>,
                    }
                    #[doc = "Description: \"Disable some set of codes and/or all codes associated with a set of users\""]
                    pub fn main(token: &String, input: MainInput) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("https://bsky.social/xrpc/com.atproto.admin.disableInviteCodes")
                            .header("Authorization", token)
                            .json(&input)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod get_invite_codes {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub cursor: Option<String>,
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
                            .get("https://bsky.social/xrpc/com.atproto.admin.getInviteCodes")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_moderation_action {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[doc = "Description: \"View details about a moderation action.\""]
                    pub fn main(
                        token: &String,
                        id: i64,
                    ) -> Result<lexicon::com::atproto::admin::defs::ActionViewDetail, reqwest::Error>
                    {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("https://bsky.social/xrpc/com.atproto.admin.getModerationAction")
                            .header("Authorization", token)
                            .send()?
                            .json::<lexicon::com::atproto::admin::defs::ActionViewDetail>();
                    }
                }
                pub mod get_moderation_actions {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub actions: Vec<lexicon::com::atproto::admin::defs::ActionView>,
                        pub cursor: Option<String>,
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
                            .get("https://bsky.social/xrpc/com.atproto.admin.getModerationActions")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_moderation_report {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[doc = "Description: \"View details about a moderation report.\""]
                    pub fn main(
                        token: &String,
                        id: i64,
                    ) -> Result<lexicon::com::atproto::admin::defs::ReportViewDetail, reqwest::Error>
                    {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("https://bsky.social/xrpc/com.atproto.admin.getModerationReport")
                            .header("Authorization", token)
                            .send()?
                            .json::<lexicon::com::atproto::admin::defs::ReportViewDetail>();
                    }
                }
                pub mod get_moderation_reports {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub reports: Vec<lexicon::com::atproto::admin::defs::ReportView>,
                        pub cursor: Option<String>,
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
                            .get("https://bsky.social/xrpc/com.atproto.admin.getModerationReports")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_record {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[doc = "Description: \"View details about a record.\""]
                    pub fn main(
                        token: &String,
                        cid: String,
                        uri: String,
                    ) -> Result<lexicon::com::atproto::admin::defs::RecordViewDetail, reqwest::Error>
                    {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("https://bsky.social/xrpc/com.atproto.admin.getRecord")
                            .header("Authorization", token)
                            .send()?
                            .json::<lexicon::com::atproto::admin::defs::RecordViewDetail>();
                    }
                }
                pub mod get_repo {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[doc = "Description: \"View details about a repository.\""]
                    pub fn main(
                        token: &String,
                        did: String,
                    ) -> Result<lexicon::com::atproto::admin::defs::RepoViewDetail, reqwest::Error>
                    {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("https://bsky.social/xrpc/com.atproto.admin.getRepo")
                            .header("Authorization", token)
                            .send()?
                            .json::<lexicon::com::atproto::admin::defs::RepoViewDetail>();
                    }
                }
                pub mod resolve_moderation_reports {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub report_ids: Vec<i64>,
                        pub created_by: String,
                        pub action_id: i64,
                    }
                    #[doc = "Description: \"Resolve moderation reports by an action.\""]
                    pub fn main(
                        token: &String,
                        input: MainInput,
                    ) -> Result<lexicon::com::atproto::admin::defs::ActionView, reqwest::Error>
                    {
                        let client = reqwest::blocking::Client::new();
                        return client . post ("https://bsky.social/xrpc/com.atproto.admin.resolveModerationReports") . header ("Authorization" , token) . json (& input) . send () ? . json :: < lexicon :: com :: atproto :: admin :: defs :: ActionView > () ;
                    }
                }
                pub mod reverse_moderation_action {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub id: i64,
                        pub reason: String,
                        pub created_by: String,
                    }
                    #[doc = "Description: \"Reverse a moderation action.\""]
                    pub fn main(
                        token: &String,
                        input: MainInput,
                    ) -> Result<lexicon::com::atproto::admin::defs::ActionView, reqwest::Error>
                    {
                        let client = reqwest::blocking::Client::new();
                        return client . post ("https://bsky.social/xrpc/com.atproto.admin.reverseModerationAction") . header ("Authorization" , token) . json (& input) . send () ? . json :: < lexicon :: com :: atproto :: admin :: defs :: ActionView > () ;
                    }
                }
                pub mod search_repos {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub repos: Vec<lexicon::com::atproto::admin::defs::RepoView>,
                        pub cursor: Option<String>,
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
                            .get("https://bsky.social/xrpc/com.atproto.admin.searchRepos")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod take_moderation_action {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum MainInputsubjectType {
                        DefsRepoRef,
                        StrongRefMain,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub subject: MainInputsubjectType,
                        pub create_label_vals: Option<Vec<String>>,
                        pub subject_blob_cids: Option<Vec<String>>,
                        pub action: String,
                        pub reason: String,
                        pub negate_label_vals: Option<Vec<String>>,
                        pub created_by: String,
                    }
                    #[doc = "Description: \"Take a moderation action on a repo.\""]
                    pub fn main(
                        token: &String,
                        input: MainInput,
                    ) -> Result<lexicon::com::atproto::admin::defs::ActionView, reqwest::Error>
                    {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("https://bsky.social/xrpc/com.atproto.admin.takeModerationAction")
                            .header("Authorization", token)
                            .json(&input)
                            .send()?
                            .json::<lexicon::com::atproto::admin::defs::ActionView>();
                    }
                }
                pub mod update_account_email {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub account: String,
                        pub email: String,
                    }
                    #[doc = "Description: \"Administrative action to update an account's email\""]
                    pub fn main(token: &String, input: MainInput) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("https://bsky.social/xrpc/com.atproto.admin.updateAccountEmail")
                            .header("Authorization", token)
                            .json(&input)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod update_account_handle {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub handle: String,
                        pub did: String,
                    }
                    #[doc = "Description: \"Administrative action to update an account's handle\""]
                    pub fn main(token: &String, input: MainInput) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("https://bsky.social/xrpc/com.atproto.admin.updateAccountHandle")
                            .header("Authorization", token)
                            .json(&input)
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
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
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
                            .get("https://bsky.social/xrpc/com.atproto.identity.resolveHandle")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod update_handle {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub handle: String,
                    }
                    #[doc = "Description: \"Updates the handle of the account\""]
                    pub fn main(token: &String, input: MainInput) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("https://bsky.social/xrpc/com.atproto.identity.updateHandle")
                            .header("Authorization", token)
                            .json(&input)
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
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Label {
                        pub cid: Option<String>,
                        pub cts: String,
                        pub neg: Option<bool>,
                        pub src: String,
                        pub uri: String,
                        pub val: String,
                    }
                }
                pub mod query_labels {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub cursor: Option<String>,
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
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
                            .get("https://bsky.social/xrpc/com.atproto.label.queryLabels")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod subscribe_labels {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Labels {
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
                        pub seq: i64,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Info {
                        pub name: String,
                        pub message: Option<String>,
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
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
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
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub reason: Option<String>,
                        pub subject: MainOutputsubjectType,
                        pub id: i64,
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                        pub reported_by: String,
                        pub created_at: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum MainInputsubjectType {
                        DefsRepoRef,
                        StrongRefMain,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub subject: MainInputsubjectType,
                        pub reason: Option<String>,
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                    }
                    #[doc = "Description: \"Report a repo or a record.\""]
                    pub fn main(
                        token: &String,
                        input: MainInput,
                    ) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("https://bsky.social/xrpc/com.atproto.moderation.createReport")
                            .header("Authorization", token)
                            .json(&input)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod defs {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum ReasonType {}
                }
            }
            pub mod repo {
                #[allow(unused_imports)]
                use super::lexicon;
                pub mod apply_writes {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Update {
                        pub rkey: String,
                        pub collection: String,
                        pub value: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Create {
                        pub collection: String,
                        pub value: String,
                        pub rkey: Option<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Delete {
                        pub rkey: String,
                        pub collection: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum WritesType {
                        Create(Box<Create>),
                        Update(Box<Update>),
                        Delete(Box<Delete>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub writes: Vec<WritesType>,
                        pub swap_commit: Option<String>,
                        pub repo: String,
                        pub validate: Option<bool>,
                    }
                    #[doc = "Description: \"Apply a batch transaction of creates, updates, and deletes.\""]
                    pub fn main(token: &String, input: MainInput) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("https://bsky.social/xrpc/com.atproto.repo.applyWrites")
                            .header("Authorization", token)
                            .json(&input)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod create_record {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub cid: String,
                        pub uri: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub swap_commit: Option<String>,
                        pub collection: String,
                        pub rkey: Option<String>,
                        pub validate: Option<bool>,
                        pub repo: String,
                        pub record: String,
                    }
                    #[doc = "Description: \"Create a new record.\""]
                    pub fn main(
                        token: &String,
                        input: MainInput,
                    ) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("https://bsky.social/xrpc/com.atproto.repo.createRecord")
                            .header("Authorization", token)
                            .json(&input)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod delete_record {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub collection: String,
                        pub repo: String,
                        pub rkey: String,
                        pub swap_record: Option<String>,
                        pub swap_commit: Option<String>,
                    }
                    #[doc = "Description: \"Delete a record, or ensure it doesn't exist.\""]
                    pub fn main(token: &String, input: MainInput) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("https://bsky.social/xrpc/com.atproto.repo.deleteRecord")
                            .header("Authorization", token)
                            .json(&input)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod describe_repo {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub handle_is_correct: bool,
                        pub handle: String,
                        pub did: String,
                        pub didDoc: String,
                        pub collections: Vec<String>,
                    }
                    #[doc = "Description: \"Get information about the repo, including the list of collections.\""]
                    pub fn main(
                        token: &String,
                        repo: String,
                    ) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("https://bsky.social/xrpc/com.atproto.repo.describeRepo")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_record {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub cid: Option<String>,
                        pub uri: String,
                        pub value: String,
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
                            .get("https://bsky.social/xrpc/com.atproto.repo.getRecord")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod list_records {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Record {
                        pub value: String,
                        pub cid: String,
                        pub uri: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub cursor: Option<String>,
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
                            .get("https://bsky.social/xrpc/com.atproto.repo.listRecords")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod put_record {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub uri: String,
                        pub cid: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub repo: String,
                        pub rkey: String,
                        pub swap_record: Option<String>,
                        pub collection: String,
                        pub validate: Option<bool>,
                        pub record: String,
                        pub swap_commit: Option<String>,
                    }
                    #[doc = "Description: \"Write a record, creating or updating it as needed.\""]
                    pub fn main(
                        token: &String,
                        input: MainInput,
                    ) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("https://bsky.social/xrpc/com.atproto.repo.putRecord")
                            .header("Authorization", token)
                            .json(&input)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod strong_ref {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Main {
                        pub cid: String,
                        pub uri: String,
                    }
                }
                pub mod upload_blob {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub blob: String,
                    }
                    #[doc = "Description: \"Upload a new blob to be added to repo in a later request.\""]
                    pub fn main(token: &String, input: ()) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("https://bsky.social/xrpc/com.atproto.repo.uploadBlob")
                            .header("Authorization", token)
                            .json(&input)
                            .send()?
                            .json::<MainOutput>();
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
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub did: String,
                        pub access_jwt: String,
                        pub handle: String,
                        pub refresh_jwt: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub invite_code: Option<String>,
                        pub handle: String,
                        pub email: String,
                        pub recovery_key: Option<String>,
                        pub password: String,
                    }
                    #[doc = "Description: \"Create an account.\""]
                    pub fn main(
                        token: &String,
                        input: MainInput,
                    ) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("https://bsky.social/xrpc/com.atproto.server.createAccount")
                            .header("Authorization", token)
                            .json(&input)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod create_app_password {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub name: String,
                    }
                    #[doc = "Description: \"Create an app-specific password.\""]
                    pub fn main(
                        token: &String,
                        input: MainInput,
                    ) -> Result<AppPassword, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("https://bsky.social/xrpc/com.atproto.server.createAppPassword")
                            .header("Authorization", token)
                            .json(&input)
                            .send()?
                            .json::<AppPassword>();
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct AppPassword {
                        pub password: String,
                        pub name: String,
                        pub created_at: String,
                    }
                }
                pub mod create_invite_code {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub code: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub use_count: i64,
                        pub for_account: Option<String>,
                    }
                    #[doc = "Description: \"Create an invite code.\""]
                    pub fn main(
                        token: &String,
                        input: MainInput,
                    ) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("https://bsky.social/xrpc/com.atproto.server.createInviteCode")
                            .header("Authorization", token)
                            .json(&input)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod create_invite_codes {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub codes: Vec<AccountCodes>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub code_count: i64,
                        pub for_accounts: Option<Vec<String>>,
                        pub use_count: i64,
                    }
                    #[doc = "Description: \"Create an invite code.\""]
                    pub fn main(
                        token: &String,
                        input: MainInput,
                    ) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("https://bsky.social/xrpc/com.atproto.server.createInviteCodes")
                            .header("Authorization", token)
                            .json(&input)
                            .send()?
                            .json::<MainOutput>();
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct AccountCodes {
                        pub account: String,
                        pub codes: Vec<String>,
                    }
                }
                pub mod create_session {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub handle: String,
                        pub email: Option<String>,
                        pub refresh_jwt: String,
                        pub did: String,
                        pub access_jwt: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub identifier: String,
                        pub password: String,
                    }
                    #[doc = "Description: \"Create an authentication session.\""]
                    pub fn main(
                        token: &String,
                        input: MainInput,
                    ) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("https://bsky.social/xrpc/com.atproto.server.createSession")
                            .header("Authorization", token)
                            .json(&input)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod defs {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct InviteCode {
                        pub code: String,
                        pub available: i64,
                        pub for_account: String,
                        pub created_by: String,
                        pub disabled: bool,
                        pub created_at: String,
                        pub uses: Vec<InviteCodeUse>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct InviteCodeUse {
                        pub used_at: String,
                        pub used_by: String,
                    }
                }
                pub mod delete_account {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub password: String,
                        pub token: String,
                        pub did: String,
                    }
                    #[doc = "Description: \"Delete a user account with a token and password.\""]
                    pub fn main(token: &String, input: MainInput) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("https://bsky.social/xrpc/com.atproto.server.deleteAccount")
                            .header("Authorization", token)
                            .json(&input)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod delete_session {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[doc = "Description: \"Delete the current session.\""]
                    pub fn main(token: &String, input: ()) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("https://bsky.social/xrpc/com.atproto.server.deleteSession")
                            .header("Authorization", token)
                            .json(&input)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod describe_server {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Links {
                        pub privacy_policy: Option<String>,
                        pub terms_of_service: Option<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub invite_code_required: Option<bool>,
                        pub links: Option<Links>,
                        pub available_user_domains: Vec<String>,
                    }
                    #[doc = "Description: \"Get a document describing the service's accounts configuration.\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("https://bsky.social/xrpc/com.atproto.server.describeServer")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_account_invite_codes {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
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
                            .get(
                                "https://bsky.social/xrpc/com.atproto.server.getAccountInviteCodes",
                            )
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_session {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub handle: String,
                        pub email: Option<String>,
                        pub did: String,
                    }
                    #[doc = "Description: \"Get information about the current session.\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("https://bsky.social/xrpc/com.atproto.server.getSession")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod list_app_passwords {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct AppPassword {
                        pub name: String,
                        pub created_at: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub passwords: Vec<AppPassword>,
                    }
                    #[doc = "Description: \"List all app-specific passwords.\""]
                    pub fn main(token: &String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("https://bsky.social/xrpc/com.atproto.server.listAppPasswords")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod refresh_session {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub handle: String,
                        pub access_jwt: String,
                        pub refresh_jwt: String,
                        pub did: String,
                    }
                    #[doc = "Description: \"Refresh an authentication session.\""]
                    pub fn main(token: &String, input: ()) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("https://bsky.social/xrpc/com.atproto.server.refreshSession")
                            .header("Authorization", token)
                            .json(&input)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod request_account_delete {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[doc = "Description: \"Initiate a user account deletion via email.\""]
                    pub fn main(token: &String, input: ()) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post(
                                "https://bsky.social/xrpc/com.atproto.server.requestAccountDelete",
                            )
                            .header("Authorization", token)
                            .json(&input)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod request_password_reset {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub email: String,
                    }
                    #[doc = "Description: \"Initiate a user account password reset via email.\""]
                    pub fn main(token: &String, input: MainInput) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post(
                                "https://bsky.social/xrpc/com.atproto.server.requestPasswordReset",
                            )
                            .header("Authorization", token)
                            .json(&input)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod reset_password {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub token: String,
                        pub password: String,
                    }
                    #[doc = "Description: \"Reset a user account password using a token.\""]
                    pub fn main(token: &String, input: MainInput) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("https://bsky.social/xrpc/com.atproto.server.resetPassword")
                            .header("Authorization", token)
                            .json(&input)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod revoke_app_password {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub name: String,
                    }
                    #[doc = "Description: \"Revoke an app-specific password by name.\""]
                    pub fn main(token: &String, input: MainInput) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .post("https://bsky.social/xrpc/com.atproto.server.revokeAppPassword")
                            .header("Authorization", token)
                            .json(&input)
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
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[doc = "Description: \"Get a blob associated with a given repo.\""]
                    pub fn main(
                        token: &String,
                        cid: String,
                        did: String,
                    ) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("https://bsky.social/xrpc/com.atproto.sync.getBlob")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod get_blocks {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[doc = "Description: \"Gets blocks from a given repo.\""]
                    pub fn main(
                        token: &String,
                        cids: Vec<String>,
                        did: String,
                    ) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("https://bsky.social/xrpc/com.atproto.sync.getBlocks")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod get_checkout {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[doc = "Description: \"Gets the repo state.\""]
                    pub fn main(
                        token: &String,
                        commit: String,
                        did: String,
                    ) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("https://bsky.social/xrpc/com.atproto.sync.getCheckout")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod get_commit_path {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
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
                            .get("https://bsky.social/xrpc/com.atproto.sync.getCommitPath")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_head {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub root: String,
                    }
                    #[doc = "Description: \"Gets the current HEAD CID of a repo.\""]
                    pub fn main(token: &String, did: String) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("https://bsky.social/xrpc/com.atproto.sync.getHead")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod get_record {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
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
                            .get("https://bsky.social/xrpc/com.atproto.sync.getRecord")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod get_repo {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[doc = "Description: \"Gets the repo state.\""]
                    pub fn main(
                        token: &String,
                        did: String,
                        earliest: String,
                        latest: String,
                    ) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("https://bsky.social/xrpc/com.atproto.sync.getRepo")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod list_blobs {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
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
                            .get("https://bsky.social/xrpc/com.atproto.sync.listBlobs")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod list_repos {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Repo {
                        pub head: String,
                        pub did: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub cursor: Option<String>,
                        pub repos: Vec<Repo>,
                    }
                    #[doc = "Description: \"List dids and root cids of hosted repos\""]
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("https://bsky.social/xrpc/com.atproto.sync.listRepos")
                            .header("Authorization", token)
                            .send()?
                            .json::<MainOutput>();
                    }
                }
                pub mod notify_of_update {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[doc = "Description: \"Notify a crawling service of a recent update. Often when a long break between updates causes the connection with the crawling service to break.\""]
                    pub fn main(token: &String, hostname: String) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("https://bsky.social/xrpc/com.atproto.sync.notifyOfUpdate")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod request_crawl {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[doc = "Description: \"Request a service to persistently crawl hosted repos.\""]
                    pub fn main(token: &String, hostname: String) -> Result<(), reqwest::Error> {
                        let client = reqwest::blocking::Client::new();
                        return client
                            .get("https://bsky.social/xrpc/com.atproto.sync.requestCrawl")
                            .header("Authorization", token)
                            .send()?
                            .json::<()>();
                    }
                }
                pub mod subscribe_repos {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Tombstone {
                        pub time: String,
                        pub seq: i64,
                        pub did: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RepoOp {
                        pub cid: Option<String>,
                        pub action: String,
                        pub path: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Commit {
                        pub commit: String,
                        pub seq: i64,
                        pub rebase: bool,
                        pub too_big: bool,
                        pub blobs: Vec<Unimplemented>,
                        pub prev: Option<String>,
                        pub repo: String,
                        pub blocks: String,
                        pub ops: Vec<RepoOp>,
                        pub time: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Handle {
                        pub time: String,
                        pub seq: i64,
                        pub handle: String,
                        pub did: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Migrate {
                        pub time: String,
                        pub migrate_to: Option<String>,
                        pub did: String,
                        pub seq: i64,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Info {
                        pub name: String,
                        pub message: Option<String>,
                    }
                }
            }
        }
    }
}

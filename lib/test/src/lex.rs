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
                    pub struct ViewerState {
                        pub blocked_by: Option<bool>,
                        pub blocking: Option<String>,
                        pub muted: Option<bool>,
                        pub followed_by: Option<String>,
                        pub following: Option<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ProfileViewBasic {
                        pub avatar: Option<String>,
                        pub did: String,
                        pub display_name: Option<String>,
                        pub handle: String,
                        pub viewer: Option<ViewerState>,
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ProfileView {
                        pub did: String,
                        pub description: Option<String>,
                        pub handle: String,
                        pub avatar: Option<String>,
                        pub viewer: Option<ViewerState>,
                        pub display_name: Option<String>,
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        pub indexed_at: Option<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ProfileViewDetailed {
                        pub follows_count: Option<i64>,
                        pub posts_count: Option<i64>,
                        pub handle: String,
                        pub banner: Option<String>,
                        pub description: Option<String>,
                        pub avatar: Option<String>,
                        pub followers_count: Option<i64>,
                        pub did: String,
                        pub indexed_at: Option<String>,
                        pub viewer: Option<ViewerState>,
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        pub display_name: Option<String>,
                    }
                }
                pub mod get_profile {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        actor: String,
                    ) -> Result<lexicon::app::bsky::actor::defs::ProfileViewDetailed, XrpcError>
                    {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.actor.getProfile".to_string(),
                        )
                        .token(token);
                        query.execute::<lexicon::app::bsky::actor::defs::ProfileViewDetailed>()
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        actors: Vec<String>,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.actor.getProfiles".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.actor.getSuggestions".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                        term: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.actor.searchActors".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        limit: i64,
                        term: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.actor.searchActorsTypeahead"
                                .to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                    pub struct External {
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
                    pub struct ViewExternal {
                        pub description: String,
                        pub thumb: Option<String>,
                        pub title: String,
                        pub uri: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Main {
                        pub external: External,
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
                    pub struct Image {
                        pub image: String,
                        pub alt: String,
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
                        pub fullsize: String,
                        pub thumb: String,
                        pub alt: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Main {
                        pub images: Vec<Image>,
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
                        pub embeds: Option<Vec<EmbedsType>>,
                        pub indexed_at: String,
                        pub author: lexicon::app::bsky::actor::defs::ProfileViewBasic,
                        pub cid: String,
                        pub uri: String,
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        pub value: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ViewBlocked {
                        pub uri: String,
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
                        pub media: ViewmediaType,
                        pub record: lexicon::app::bsky::embed::record::View,
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
                    #[serde(rename_all = "camelCase")]
                    pub struct ThreadViewPost {
                        pub parent: Option<ThreadViewPostparentType>,
                        pub post: PostView,
                        pub replies: Option<Vec<RepliesType>>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ViewerState {
                        pub repost: Option<String>,
                        pub like: Option<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct BlockedPost {
                        pub uri: String,
                        pub blocked: bool,
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
                        pub reply: Option<ReplyRef>,
                        pub reason: Option<FeedViewPostreasonType>,
                        pub post: lexicon::app::bsky::feed::defs::PostView,
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
                        pub cid: String,
                        pub embed: Option<PostViewembedType>,
                        pub like_count: Option<i64>,
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        pub record: String,
                        pub repost_count: Option<i64>,
                        pub reply_count: Option<i64>,
                        pub author: lexicon::app::bsky::actor::defs::ProfileViewBasic,
                        pub viewer: Option<ViewerState>,
                        pub uri: String,
                        pub indexed_at: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct NotFoundPost {
                        pub uri: String,
                        pub not_found: bool,
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        actor: String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.feed.getAuthorFeed".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                        pub cursor: Option<String>,
                        pub cid: Option<String>,
                        pub uri: String,
                        pub likes: Vec<Like>,
                    }
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        cid: String,
                        cursor: String,
                        limit: i64,
                        uri: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.feed.getLikes".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Like {
                        pub indexed_at: String,
                        pub actor: lexicon::app::bsky::actor::defs::ProfileView,
                        pub created_at: String,
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        depth: i64,
                        uri: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.feed.getPostThread".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        uris: Vec<String>,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.feed.getPosts".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                        pub cid: Option<String>,
                        pub cursor: Option<String>,
                        pub reposted_by: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                        pub uri: String,
                    }
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        cid: String,
                        cursor: String,
                        limit: i64,
                        uri: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.feed.getRepostedBy".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        algorithm: String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.feed.getTimeline".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                        pub root: lexicon::com::atproto::repo::strong_ref::Main,
                        pub parent: lexicon::com::atproto::repo::strong_ref::Main,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Entity {
                        pub value: String,
                        pub r#type: String,
                        pub index: TextSlice,
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
                        pub cursor: Option<String>,
                        pub blocks: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                    }
                    #[doc = "Description: \"Who is the requester's account blocking?\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.graph.getBlocks".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                        pub cursor: Option<String>,
                        pub followers: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                        pub subject: lexicon::app::bsky::actor::defs::ProfileView,
                    }
                    #[doc = "Description: \"Who is following an actor?\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        actor: String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.graph.getFollowers".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                        pub follows: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                        pub cursor: Option<String>,
                        pub subject: lexicon::app::bsky::actor::defs::ProfileView,
                    }
                    #[doc = "Description: \"Who is an actor following?\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        actor: String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.graph.getFollows".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.graph.getMutes".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(token: &String, seen_at: String) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.notification.getUnreadCount"
                                .to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                        pub cursor: Option<String>,
                        pub notifications: Vec<Notification>,
                    }
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                        seen_at: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.notification.listNotifications"
                                .to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Notification {
                        pub record: String,
                        pub is_read: bool,
                        pub cid: String,
                        pub reason: String,
                        pub indexed_at: String,
                        pub author: lexicon::app::bsky::actor::defs::ProfileView,
                        pub uri: String,
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        pub reason_subject: Option<String>,
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
                    pub struct Mention {
                        pub did: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ByteSlice {
                        pub byte_start: i64,
                        pub byte_end: i64,
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
                        pub cursor: Option<String>,
                        pub feed: Vec<lexicon::app::bsky::feed::defs::FeedViewPost>,
                    }
                    #[doc = "Description: \"An unspecced view of globally popular items\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        cursor: String,
                        include_nsfw: bool,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.unspecced.getPopular".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                        pub created_at: String,
                        pub created_by: String,
                        pub reason: String,
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
                    pub struct ImageDetails {
                        pub width: i64,
                        pub height: i64,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct VideoDetails {
                        pub height: i64,
                        pub width: i64,
                        pub length: i64,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RepoView {
                        pub invited_by: Option<lexicon::com::atproto::server::defs::InviteCode>,
                        pub handle: String,
                        pub email: Option<String>,
                        pub related_records: Vec<Unimplemented>,
                        pub did: String,
                        pub moderation: Moderation,
                        pub indexed_at: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub enum ActionType {}
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
                        pub details: Option<BlobViewdetailsType>,
                        pub cid: String,
                        pub created_at: String,
                        pub size: i64,
                        pub moderation: Option<Moderation>,
                        pub mime_type: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RecordViewDetail {
                        pub value: String,
                        pub moderation: ModerationDetail,
                        pub blobs: Vec<BlobView>,
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        pub cid: String,
                        pub indexed_at: String,
                        pub uri: String,
                        pub repo: RepoView,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ModerationDetail {
                        pub current_action: Option<ActionViewCurrent>,
                        pub reports: Vec<ReportView>,
                        pub actions: Vec<ActionView>,
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
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                        pub reason: Option<String>,
                        pub reported_by: String,
                        pub created_at: String,
                        pub subject: ReportViewsubjectType,
                        pub resolved_by_action_ids: Vec<i64>,
                        pub id: i64,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RepoViewDetail {
                        pub moderation: ModerationDetail,
                        pub handle: String,
                        pub email: Option<String>,
                        pub did: String,
                        pub indexed_at: String,
                        pub related_records: Vec<Unimplemented>,
                        pub invites: Option<Vec<lexicon::com::atproto::server::defs::InviteCode>>,
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        pub invited_by: Option<lexicon::com::atproto::server::defs::InviteCode>,
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
                        pub subject_blobs: Vec<BlobView>,
                        pub reversal: Option<ActionReversal>,
                        pub action: ActionType,
                        pub id: i64,
                        pub reason: String,
                        pub subject: ActionViewDetailsubjectType,
                        pub created_by: String,
                        pub create_label_vals: Option<Vec<String>>,
                        pub created_at: String,
                        pub resolved_reports: Vec<ReportView>,
                        pub negate_label_vals: Option<Vec<String>>,
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
                        pub id: i64,
                        pub resolved_report_ids: Vec<i64>,
                        pub created_at: String,
                        pub reversal: Option<ActionReversal>,
                        pub subject_blob_cids: Vec<String>,
                        pub create_label_vals: Option<Vec<String>>,
                        pub action: ActionType,
                        pub subject: ActionViewsubjectType,
                        pub reason: String,
                        pub negate_label_vals: Option<Vec<String>>,
                        pub created_by: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RecordView {
                        pub repo: RepoView,
                        pub blob_cids: Vec<String>,
                        pub indexed_at: String,
                        pub uri: String,
                        pub cid: String,
                        pub moderation: Moderation,
                        pub value: String,
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
                    pub enum ReportViewDetailsubjectType {
                        RepoView(Box<RepoView>),
                        RecordView(Box<RecordView>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ReportViewDetail {
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                        pub reported_by: String,
                        pub id: i64,
                        pub reason: Option<String>,
                        pub created_at: String,
                        pub resolved_by_actions:
                            Vec<lexicon::com::atproto::admin::defs::ActionView>,
                        pub subject: ReportViewDetailsubjectType,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ActionViewCurrent {
                        pub id: i64,
                        pub action: ActionType,
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
                        pub codes: Option<Vec<String>>,
                        pub accounts: Option<Vec<String>>,
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                        sort: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.admin.getInviteCodes".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        id: i64,
                    ) -> Result<lexicon::com::atproto::admin::defs::ActionViewDetail, XrpcError>
                    {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.admin.getModerationAction"
                                .to_string(),
                        )
                        .token(token);
                        query.execute::<lexicon::com::atproto::admin::defs::ActionViewDetail>()
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
                        pub cursor: Option<String>,
                        pub actions: Vec<lexicon::com::atproto::admin::defs::ActionView>,
                    }
                    #[doc = "Description: \"List moderation actions related to a subject.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                        subject: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.admin.getModerationActions"
                                .to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        id: i64,
                    ) -> Result<lexicon::com::atproto::admin::defs::ReportViewDetail, XrpcError>
                    {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.admin.getModerationReport"
                                .to_string(),
                        )
                        .token(token);
                        query.execute::<lexicon::com::atproto::admin::defs::ReportViewDetail>()
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                        resolved: bool,
                        subject: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.admin.getModerationReports"
                                .to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        cid: String,
                        uri: String,
                    ) -> Result<lexicon::com::atproto::admin::defs::RecordViewDetail, XrpcError>
                    {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.admin.getRecord".to_string(),
                        )
                        .token(token);
                        query.execute::<lexicon::com::atproto::admin::defs::RecordViewDetail>()
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        did: String,
                    ) -> Result<lexicon::com::atproto::admin::defs::RepoViewDetail, XrpcError>
                    {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.admin.getRepo".to_string(),
                        )
                        .token(token);
                        query.execute::<lexicon::com::atproto::admin::defs::RepoViewDetail>()
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
                        pub created_by: String,
                        pub action_id: i64,
                        pub report_ids: Vec<i64>,
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
                        pub cursor: Option<String>,
                        pub repos: Vec<lexicon::com::atproto::admin::defs::RepoView>,
                    }
                    #[doc = "Description: \"Find repositories based on a search term.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        cursor: String,
                        invited_by: String,
                        limit: i64,
                        term: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.admin.searchRepos".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                        pub subject_blob_cids: Option<Vec<String>>,
                        pub negate_label_vals: Option<Vec<String>>,
                        pub action: String,
                        pub reason: String,
                        pub created_by: String,
                        pub create_label_vals: Option<Vec<String>>,
                        pub subject: MainInputsubjectType,
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
                        pub email: String,
                        pub account: String,
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
                        pub did: String,
                        pub handle: String,
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(token: &String, handle: String) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.identity.resolveHandle"
                                .to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                        pub neg: Option<bool>,
                        pub cts: String,
                        pub uri: String,
                        pub cid: Option<String>,
                        pub val: String,
                        pub src: String,
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
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
                        pub cursor: Option<String>,
                    }
                    #[doc = "Description: \"Find labels relevant to the provided URI patterns.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                        sources: Vec<String>,
                        uri_patterns: Vec<String>,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.label.queryLabels".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                        pub subject: MainOutputsubjectType,
                        pub reported_by: String,
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                        pub created_at: String,
                        pub id: i64,
                        pub reason: Option<String>,
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
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                        pub reason: Option<String>,
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
                        pub collection: String,
                        pub value: String,
                        pub rkey: String,
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
                    #[serde(rename_all = "camelCase")]
                    pub struct Create {
                        pub rkey: Option<String>,
                        pub collection: String,
                        pub value: String,
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
                        pub validate: Option<bool>,
                        pub swap_commit: Option<String>,
                        pub writes: Vec<WritesType>,
                        pub repo: String,
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
                        pub collection: String,
                        pub repo: String,
                        pub validate: Option<bool>,
                        pub record: String,
                        pub swap_commit: Option<String>,
                        pub rkey: Option<String>,
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
                        pub rkey: String,
                        pub swap_record: Option<String>,
                        pub collection: String,
                        pub swap_commit: Option<String>,
                        pub repo: String,
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
                        pub did: String,
                        pub collections: Vec<String>,
                        pub handle_is_correct: bool,
                        pub handle: String,
                        pub didDoc: String,
                    }
                    #[doc = "Description: \"Get information about the repo, including the list of collections.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(token: &String, repo: String) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.repo.describeRepo".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                        pub value: String,
                        pub uri: String,
                        pub cid: Option<String>,
                    }
                    #[doc = "Description: \"Get a record.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        cid: String,
                        collection: String,
                        repo: String,
                        rkey: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.repo.getRecord".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                    pub struct MainOutput {
                        pub cursor: Option<String>,
                        pub records: Vec<Record>,
                    }
                    #[doc = "Description: \"List a range of records in a collection.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        collection: String,
                        cursor: String,
                        limit: i64,
                        repo: String,
                        reverse: bool,
                        rkey_end: String,
                        rkey_start: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.repo.listRecords".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Record {
                        pub cid: String,
                        pub value: String,
                        pub uri: String,
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
                        pub validate: Option<bool>,
                        pub record: String,
                        pub rkey: String,
                        pub repo: String,
                        pub collection: String,
                        pub swap_record: Option<String>,
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
                        pub uri: String,
                        pub cid: String,
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
                        pub handle: String,
                        pub did: String,
                        pub refresh_jwt: String,
                        pub access_jwt: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub invite_code: Option<String>,
                        pub recovery_key: Option<String>,
                        pub email: String,
                        pub handle: String,
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
                        pub name: String,
                        pub password: String,
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
                        pub for_account: Option<String>,
                        pub use_count: i64,
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
                        pub for_accounts: Option<Vec<String>>,
                        pub code_count: i64,
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
                        pub codes: Vec<String>,
                        pub account: String,
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
                        pub access_jwt: String,
                        pub refresh_jwt: String,
                        pub did: String,
                        pub handle: String,
                        pub email: Option<String>,
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
                        pub created_by: String,
                        pub created_at: String,
                        pub for_account: String,
                        pub uses: Vec<InviteCodeUse>,
                        pub available: i64,
                        pub disabled: bool,
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
                        pub did: String,
                        pub token: String,
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
                        pub terms_of_service: Option<String>,
                        pub privacy_policy: Option<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub available_user_domains: Vec<String>,
                        pub links: Option<Links>,
                        pub invite_code_required: Option<bool>,
                    }
                    #[doc = "Description: \"Get a document describing the service's accounts configuration.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(token: &String) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.server.describeServer"
                                .to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        create_available: bool,
                        include_used: bool,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.server.getAccountInviteCodes"
                                .to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                        pub did: String,
                        pub handle: String,
                        pub email: Option<String>,
                    }
                    #[doc = "Description: \"Get information about the current session.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(token: &String) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.server.getSession".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                    pub struct MainOutput {
                        pub passwords: Vec<AppPassword>,
                    }
                    #[doc = "Description: \"List all app-specific passwords.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(token: &String) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.server.listAppPasswords"
                                .to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct AppPassword {
                        pub name: String,
                        pub created_at: String,
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
                        pub did: String,
                        pub access_jwt: String,
                        pub refresh_jwt: String,
                        pub handle: String,
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(token: &String, cid: String, did: String) -> Result<(), XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.getBlob".to_string(),
                        )
                        .token(token);
                        query.execute::<()>()
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        cids: Vec<String>,
                        did: String,
                    ) -> Result<(), XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.getBlocks".to_string(),
                        )
                        .token(token);
                        query.execute::<()>()
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        commit: String,
                        did: String,
                    ) -> Result<(), XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.getCheckout".to_string(),
                        )
                        .token(token);
                        query.execute::<()>()
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        did: String,
                        earliest: String,
                        latest: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.getCommitPath".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(token: &String, did: String) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.getHead".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        collection: String,
                        commit: String,
                        did: String,
                        rkey: String,
                    ) -> Result<(), XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.getRecord".to_string(),
                        )
                        .token(token);
                        query.execute::<()>()
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        did: String,
                        earliest: String,
                        latest: String,
                    ) -> Result<(), XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.getRepo".to_string(),
                        )
                        .token(token);
                        query.execute::<()>()
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        did: String,
                        earliest: String,
                        latest: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.listBlobs".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                        pub did: String,
                        pub head: String,
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.listRepos".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>()
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(token: &String, hostname: String) -> Result<(), XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.notifyOfUpdate".to_string(),
                        )
                        .token(token);
                        query.execute::<()>()
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(token: &String, hostname: String) -> Result<(), XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.requestCrawl".to_string(),
                        )
                        .token(token);
                        query.execute::<()>()
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
                    pub struct Migrate {
                        pub did: String,
                        pub time: String,
                        pub migrate_to: Option<String>,
                        pub seq: i64,
                    }
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
                    pub struct Info {
                        pub message: Option<String>,
                        pub name: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Handle {
                        pub seq: i64,
                        pub time: String,
                        pub did: String,
                        pub handle: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Commit {
                        pub ops: Vec<RepoOp>,
                        pub too_big: bool,
                        pub seq: i64,
                        pub rebase: bool,
                        pub prev: Option<String>,
                        pub time: String,
                        pub blocks: String,
                        pub commit: String,
                        pub blobs: Vec<Unimplemented>,
                        pub repo: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RepoOp {
                        pub action: String,
                        pub path: String,
                        pub cid: Option<String>,
                    }
                }
            }
        }
    }
}

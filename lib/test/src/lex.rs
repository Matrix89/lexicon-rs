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
                        pub followed_by: Option<String>,
                        pub muted: Option<bool>,
                        pub blocking: Option<String>,
                        pub following: Option<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ProfileViewBasic {
                        pub avatar: Option<String>,
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        pub did: String,
                        pub handle: String,
                        pub viewer: Option<ViewerState>,
                        pub display_name: Option<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ProfileViewDetailed {
                        pub banner: Option<String>,
                        pub description: Option<String>,
                        pub follows_count: Option<i64>,
                        pub indexed_at: Option<String>,
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        pub posts_count: Option<i64>,
                        pub did: String,
                        pub viewer: Option<ViewerState>,
                        pub handle: String,
                        pub display_name: Option<String>,
                        pub followers_count: Option<i64>,
                        pub avatar: Option<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ProfileView {
                        pub handle: String,
                        pub description: Option<String>,
                        pub indexed_at: Option<String>,
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        pub display_name: Option<String>,
                        pub avatar: Option<String>,
                        pub did: String,
                        pub viewer: Option<ViewerState>,
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
                        .param("actor".to_string(), actor.to_string())
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
                        pub actors: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                        pub cursor: Option<String>,
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
                        .param("limit".to_string(), limit.to_string())
                        .param("cursor".to_string(), cursor.to_string())
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
                        .param("limit".to_string(), limit.to_string())
                        .param("cursor".to_string(), cursor.to_string())
                        .param("term".to_string(), term.to_string())
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
                        .param("limit".to_string(), limit.to_string())
                        .param("term".to_string(), term.to_string())
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
                    pub struct Main {
                        pub external: External,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ViewExternal {
                        pub thumb: Option<String>,
                        pub uri: String,
                        pub title: String,
                        pub description: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct External {
                        pub description: String,
                        pub thumb: Option<String>,
                        pub uri: String,
                        pub title: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct View {
                        pub external: ViewExternal,
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
                    pub struct Main {
                        pub images: Vec<Image>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ViewImage {
                        pub thumb: String,
                        pub fullsize: String,
                        pub alt: String,
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
                    #[serde(rename_all = "camelCase")]
                    pub struct ViewBlocked {
                        pub uri: String,
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
                    pub struct Main {
                        pub record: lexicon::com::atproto::repo::strong_ref::Main,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(tag = "$type")]
                    pub enum ViewrecordType {
                        #[serde(alias = "app.bsky.embed.record#viewRecord")]
                        ViewRecord(Box<ViewRecord>),
                        #[serde(alias = "app.bsky.embed.record#viewNotFound")]
                        ViewNotFound(Box<ViewNotFound>),
                        #[serde(alias = "app.bsky.embed.record#viewBlocked")]
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
                    #[serde(tag = "$type")]
                    pub enum EmbedsType {
                        #[serde(alias = "app.bsky.embed.images#view")]
                        ImagesView(Box<lexicon::app::bsky::embed::images::View>),
                        #[serde(alias = "app.bsky.embed.external#view")]
                        ExternalView(Box<lexicon::app::bsky::embed::external::View>),
                        #[serde(alias = "app.bsky.embed.record#view")]
                        RecordView(Box<lexicon::app::bsky::embed::record::View>),
                        #[serde(alias = "app.bsky.embed.recordWithMedia#view")]
                        RecordWithMediaView(
                            Box<lexicon::app::bsky::embed::record_with_media::View>,
                        ),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ViewRecord {
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        pub cid: String,
                        pub embeds: Option<Vec<EmbedsType>>,
                        pub indexed_at: String,
                        pub author: lexicon::app::bsky::actor::defs::ProfileViewBasic,
                        pub uri: String,
                        pub value: ::serde_json::Value,
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
                    #[serde(tag = "$type")]
                    pub enum MainmediaType {
                        #[serde(alias = "app.bsky.embed.images")]
                        Images(Box<lexicon::app::bsky::embed::images::Main>),
                        #[serde(alias = "app.bsky.embed.external")]
                        External(Box<lexicon::app::bsky::embed::external::Main>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Main {
                        pub media: MainmediaType,
                        pub record: lexicon::app::bsky::embed::record::Main,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(tag = "$type")]
                    pub enum ViewmediaType {
                        #[serde(alias = "app.bsky.embed.images#view")]
                        ImagesView(Box<lexicon::app::bsky::embed::images::View>),
                        #[serde(alias = "app.bsky.embed.external#view")]
                        ExternalView(Box<lexicon::app::bsky::embed::external::View>),
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
                    #[serde(rename_all = "camelCase")]
                    pub struct ReplyRef {
                        pub parent: lexicon::app::bsky::feed::defs::PostView,
                        pub root: lexicon::app::bsky::feed::defs::PostView,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ReasonRepost {
                        pub indexed_at: String,
                        pub by: lexicon::app::bsky::actor::defs::ProfileViewBasic,
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
                        pub not_found: bool,
                        pub uri: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(tag = "$type")]
                    pub enum ThreadViewPostparentType {
                        #[serde(alias = "app.bsky.feed.defs#threadViewPost")]
                        ThreadViewPost(Box<ThreadViewPost>),
                        #[serde(alias = "app.bsky.feed.defs#notFoundPost")]
                        NotFoundPost(Box<NotFoundPost>),
                        #[serde(alias = "app.bsky.feed.defs#blockedPost")]
                        BlockedPost(Box<BlockedPost>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(tag = "$type")]
                    pub enum RepliesType {
                        #[serde(alias = "app.bsky.feed.defs#threadViewPost")]
                        ThreadViewPost(Box<ThreadViewPost>),
                        #[serde(alias = "app.bsky.feed.defs#notFoundPost")]
                        NotFoundPost(Box<NotFoundPost>),
                        #[serde(alias = "app.bsky.feed.defs#blockedPost")]
                        BlockedPost(Box<BlockedPost>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ThreadViewPost {
                        pub parent: Option<ThreadViewPostparentType>,
                        pub replies: Option<Vec<RepliesType>>,
                        pub post: PostView,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(tag = "$type")]
                    pub enum FeedViewPostreasonType {
                        #[serde(alias = "app.bsky.feed.defs#reasonRepost")]
                        ReasonRepost(Box<ReasonRepost>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct FeedViewPost {
                        pub post: lexicon::app::bsky::feed::defs::PostView,
                        pub reason: Option<FeedViewPostreasonType>,
                        pub reply: Option<ReplyRef>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(tag = "$type")]
                    pub enum PostViewembedType {
                        #[serde(alias = "app.bsky.embed.images#view")]
                        ImagesView(Box<lexicon::app::bsky::embed::images::View>),
                        #[serde(alias = "app.bsky.embed.external#view")]
                        ExternalView(Box<lexicon::app::bsky::embed::external::View>),
                        #[serde(alias = "app.bsky.embed.record#view")]
                        RecordView(Box<lexicon::app::bsky::embed::record::View>),
                        #[serde(alias = "app.bsky.embed.recordWithMedia#view")]
                        RecordWithMediaView(
                            Box<lexicon::app::bsky::embed::record_with_media::View>,
                        ),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct PostView {
                        pub like_count: Option<i64>,
                        pub indexed_at: String,
                        pub embed: Option<PostViewembedType>,
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        pub uri: String,
                        pub cid: String,
                        pub author: lexicon::app::bsky::actor::defs::ProfileViewBasic,
                        pub record: ::serde_json::Value,
                        pub reply_count: Option<i64>,
                        pub repost_count: Option<i64>,
                        pub viewer: Option<ViewerState>,
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
                        .param("cursor".to_string(), cursor.to_string())
                        .param("limit".to_string(), limit.to_string())
                        .param("actor".to_string(), actor.to_string())
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
                        pub likes: Vec<Like>,
                        pub cid: Option<String>,
                        pub uri: String,
                        pub cursor: Option<String>,
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
                        .param("cursor".to_string(), cursor.to_string())
                        .param("uri".to_string(), uri.to_string())
                        .param("limit".to_string(), limit.to_string())
                        .param("cid".to_string(), cid.to_string())
                        .token(token);
                        query.execute::<MainOutput>()
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Like {
                        pub created_at: String,
                        pub indexed_at: String,
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
                    #[serde(tag = "$type")]
                    pub enum MainOutputthreadType {
                        #[serde(alias = "app.bsky.feed.defs#threadViewPost")]
                        DefsThreadViewPost(Box<lexicon::app::bsky::feed::defs::ThreadViewPost>),
                        #[serde(alias = "app.bsky.feed.defs#notFoundPost")]
                        DefsNotFoundPost(Box<lexicon::app::bsky::feed::defs::NotFoundPost>),
                        #[serde(alias = "app.bsky.feed.defs#blockedPost")]
                        DefsBlockedPost(Box<lexicon::app::bsky::feed::defs::BlockedPost>),
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
                        .param("depth".to_string(), depth.to_string())
                        .param("uri".to_string(), uri.to_string())
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
                        pub reposted_by: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                        pub cid: Option<String>,
                        pub cursor: Option<String>,
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
                        .param("limit".to_string(), limit.to_string())
                        .param("uri".to_string(), uri.to_string())
                        .param("cursor".to_string(), cursor.to_string())
                        .param("cid".to_string(), cid.to_string())
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
                        pub feed: Vec<lexicon::app::bsky::feed::defs::FeedViewPost>,
                        pub cursor: Option<String>,
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
                        .param("limit".to_string(), limit.to_string())
                        .param("cursor".to_string(), cursor.to_string())
                        .param("algorithm".to_string(), algorithm.to_string())
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
                        pub parent: lexicon::com::atproto::repo::strong_ref::Main,
                        pub root: lexicon::com::atproto::repo::strong_ref::Main,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Entity {
                        pub index: TextSlice,
                        pub r#type: String,
                        pub value: String,
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
                        .param("cursor".to_string(), cursor.to_string())
                        .param("limit".to_string(), limit.to_string())
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
                        pub subject: lexicon::app::bsky::actor::defs::ProfileView,
                        pub followers: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
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
                        .param("actor".to_string(), actor.to_string())
                        .param("limit".to_string(), limit.to_string())
                        .param("cursor".to_string(), cursor.to_string())
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
                        pub subject: lexicon::app::bsky::actor::defs::ProfileView,
                        pub cursor: Option<String>,
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
                        .param("limit".to_string(), limit.to_string())
                        .param("cursor".to_string(), cursor.to_string())
                        .param("actor".to_string(), actor.to_string())
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
                        pub cursor: Option<String>,
                        pub mutes: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
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
                        .param("cursor".to_string(), cursor.to_string())
                        .param("limit".to_string(), limit.to_string())
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
                        .param("seenAt".to_string(), seen_at.to_string())
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
                        .param("seenAt".to_string(), seen_at.to_string())
                        .param("limit".to_string(), limit.to_string())
                        .param("cursor".to_string(), cursor.to_string())
                        .token(token);
                        query.execute::<MainOutput>()
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(tag = "$type")]
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
                    #[serde(rename_all = "camelCase")]
                    pub struct Notification {
                        pub cid: String,
                        pub uri: String,
                        pub record: ::serde_json::Value,
                        pub reason: Reason,
                        pub reason_subject: Option<String>,
                        pub is_read: bool,
                        pub indexed_at: String,
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
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
                    #[serde(tag = "$type")]
                    pub enum FeaturesType {
                        #[serde(alias = "app.bsky.richtext.facet#mention")]
                        Mention(Box<Mention>),
                        #[serde(alias = "app.bsky.richtext.facet#link")]
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
                    pub struct ByteSlice {
                        pub byte_end: i64,
                        pub byte_start: i64,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Link {
                        pub uri: String,
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
                        .param("limit".to_string(), limit.to_string())
                        .param("cursor".to_string(), cursor.to_string())
                        .param("includeNsfw".to_string(), include_nsfw.to_string())
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
                        .param("sort".to_string(), sort.to_string())
                        .param("limit".to_string(), limit.to_string())
                        .param("cursor".to_string(), cursor.to_string())
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
                        .param("id".to_string(), id.to_string())
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
                        .param("limit".to_string(), limit.to_string())
                        .param("subject".to_string(), subject.to_string())
                        .param("cursor".to_string(), cursor.to_string())
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
                        .param("id".to_string(), id.to_string())
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
                        pub cursor: Option<String>,
                        pub reports: Vec<lexicon::com::atproto::admin::defs::ReportView>,
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
                        .param("resolved".to_string(), resolved.to_string())
                        .param("subject".to_string(), subject.to_string())
                        .param("limit".to_string(), limit.to_string())
                        .param("cursor".to_string(), cursor.to_string())
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
                        .param("uri".to_string(), uri.to_string())
                        .param("cid".to_string(), cid.to_string())
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
                        .param("did".to_string(), did.to_string())
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
                        pub action_id: i64,
                        pub report_ids: Vec<i64>,
                        pub created_by: String,
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
                        pub reason: String,
                        pub created_by: String,
                        pub id: i64,
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
                        .param("term".to_string(), term.to_string())
                        .param("invitedBy".to_string(), invited_by.to_string())
                        .param("limit".to_string(), limit.to_string())
                        .param("cursor".to_string(), cursor.to_string())
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
                    #[serde(tag = "$type")]
                    pub enum MainInputsubjectType {
                        #[serde(alias = "com.atproto.admin.defs#repoRef")]
                        DefsRepoRef(Box<lexicon::com::atproto::admin::defs::RepoRef>),
                        #[serde(alias = "com.atproto.repo.strongRef")]
                        StrongRef(Box<lexicon::com::atproto::repo::strong_ref::Main>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(tag = "$type")]
                    pub enum Action {
                        #[serde(alias = "com.atproto.admin.defs#takedown")]
                        DefsTakedown(Box<lexicon::com::atproto::admin::defs::Takedown>),
                        #[serde(alias = "com.atproto.admin.defs#flag")]
                        DefsFlag(Box<lexicon::com::atproto::admin::defs::Flag>),
                        #[serde(alias = "com.atproto.admin.defs#acknowledge")]
                        DefsAcknowledge(Box<lexicon::com::atproto::admin::defs::Acknowledge>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub reason: String,
                        pub subject_blob_cids: Option<Vec<String>>,
                        pub negate_label_vals: Option<Vec<String>>,
                        pub subject: MainInputsubjectType,
                        pub created_by: String,
                        pub action: Action,
                        pub create_label_vals: Option<Vec<String>>,
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
                    #[serde(tag = "$type")]
                    pub enum ActionViewsubjectType {
                        #[serde(alias = "com.atproto.admin.defs#repoRef")]
                        RepoRef(Box<RepoRef>),
                        #[serde(alias = "com.atproto.repo.strongRef")]
                        StrongRef(Box<lexicon::com::atproto::repo::strong_ref::Main>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ActionView {
                        pub reason: String,
                        pub resolved_report_ids: Vec<i64>,
                        pub create_label_vals: Option<Vec<String>>,
                        pub subject: ActionViewsubjectType,
                        pub id: i64,
                        pub action: ActionType,
                        pub created_by: String,
                        pub created_at: String,
                        pub negate_label_vals: Option<Vec<String>>,
                        pub reversal: Option<ActionReversal>,
                        pub subject_blob_cids: Vec<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(tag = "$type")]
                    pub enum BlobViewdetailsType {
                        #[serde(alias = "com.atproto.admin.defs#imageDetails")]
                        ImageDetails(Box<ImageDetails>),
                        #[serde(alias = "com.atproto.admin.defs#videoDetails")]
                        VideoDetails(Box<VideoDetails>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct BlobView {
                        pub moderation: Option<Moderation>,
                        pub mime_type: String,
                        pub created_at: String,
                        pub details: Option<BlobViewdetailsType>,
                        pub cid: String,
                        pub size: i64,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ActionViewCurrent {
                        pub id: i64,
                        pub action: ActionType,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RepoView {
                        pub email: Option<String>,
                        pub invited_by: Option<lexicon::com::atproto::server::defs::InviteCode>,
                        pub handle: String,
                        pub related_records: Vec<Unimplemented>,
                        pub moderation: Moderation,
                        pub indexed_at: String,
                        pub did: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RepoRef {
                        pub did: String,
                    }
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    pub struct Takedown;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RecordView {
                        pub blob_cids: Vec<String>,
                        pub uri: String,
                        pub value: ::serde_json::Value,
                        pub indexed_at: String,
                        pub moderation: Moderation,
                        pub repo: RepoView,
                        pub cid: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Moderation {
                        pub current_action: Option<ActionViewCurrent>,
                    }
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    pub struct Escalate;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RepoViewDetail {
                        pub email: Option<String>,
                        pub did: String,
                        pub handle: String,
                        pub related_records: Vec<Unimplemented>,
                        pub moderation: ModerationDetail,
                        pub invited_by: Option<lexicon::com::atproto::server::defs::InviteCode>,
                        pub indexed_at: String,
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        pub invites: Option<Vec<lexicon::com::atproto::server::defs::InviteCode>>,
                    }
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    pub struct Flag;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RecordViewDetail {
                        pub indexed_at: String,
                        pub uri: String,
                        pub moderation: ModerationDetail,
                        pub repo: RepoView,
                        pub value: ::serde_json::Value,
                        pub cid: String,
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        pub blobs: Vec<BlobView>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(tag = "$type")]
                    pub enum ActionViewDetailsubjectType {
                        #[serde(alias = "com.atproto.admin.defs#repoView")]
                        RepoView(Box<RepoView>),
                        #[serde(alias = "com.atproto.admin.defs#recordView")]
                        RecordView(Box<RecordView>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ActionViewDetail {
                        pub action: ActionType,
                        pub created_at: String,
                        pub resolved_reports: Vec<ReportView>,
                        pub created_by: String,
                        pub reason: String,
                        pub id: i64,
                        pub subject_blobs: Vec<BlobView>,
                        pub create_label_vals: Option<Vec<String>>,
                        pub negate_label_vals: Option<Vec<String>>,
                        pub reversal: Option<ActionReversal>,
                        pub subject: ActionViewDetailsubjectType,
                    }
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    pub struct Acknowledge;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ModerationDetail {
                        pub current_action: Option<ActionViewCurrent>,
                        pub actions: Vec<ActionView>,
                        pub reports: Vec<ReportView>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(tag = "$type")]
                    pub enum ReportViewDetailsubjectType {
                        #[serde(alias = "com.atproto.admin.defs#repoView")]
                        RepoView(Box<RepoView>),
                        #[serde(alias = "com.atproto.admin.defs#recordView")]
                        RecordView(Box<RecordView>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ReportViewDetail {
                        pub reason: Option<String>,
                        pub created_at: String,
                        pub resolved_by_actions:
                            Vec<lexicon::com::atproto::admin::defs::ActionView>,
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                        pub reported_by: String,
                        pub id: i64,
                        pub subject: ReportViewDetailsubjectType,
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
                        pub width: i64,
                        pub height: i64,
                        pub length: i64,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(tag = "$type")]
                    pub enum ReportViewsubjectType {
                        #[serde(alias = "com.atproto.admin.defs#repoRef")]
                        RepoRef(Box<RepoRef>),
                        #[serde(alias = "com.atproto.repo.strongRef")]
                        StrongRef(Box<lexicon::com::atproto::repo::strong_ref::Main>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ReportView {
                        pub reported_by: String,
                        pub created_at: String,
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                        pub reason: Option<String>,
                        pub id: i64,
                        pub resolved_by_action_ids: Vec<i64>,
                        pub subject: ReportViewsubjectType,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ActionReversal {
                        pub created_by: String,
                        pub reason: String,
                        pub created_at: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(tag = "$type")]
                    pub enum ActionType {
                        #[serde(alias = "com.atproto.admin.defs#takedown")]
                        Takedown(Box<Takedown>),
                        #[serde(alias = "com.atproto.admin.defs#flag")]
                        Flag(Box<Flag>),
                        #[serde(alias = "com.atproto.admin.defs#acknowledge")]
                        Acknowledge(Box<Acknowledge>),
                        #[serde(alias = "com.atproto.admin.defs#escalate")]
                        Escalate(Box<Escalate>),
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
                        .param("handle".to_string(), handle.to_string())
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
                        pub uri: String,
                        pub val: String,
                        pub cid: Option<String>,
                        pub neg: Option<bool>,
                        pub cts: String,
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
                        pub cursor: Option<String>,
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
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
                        .param("cursor".to_string(), cursor.to_string())
                        .param("limit".to_string(), limit.to_string())
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
                    #[serde(tag = "$type")]
                    pub enum Name {
                        OutdatedCursor,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Info {
                        pub message: Option<String>,
                        pub name: Name,
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
                    #[serde(tag = "$type")]
                    pub enum MainOutputsubjectType {
                        #[serde(alias = "com.atproto.admin.defs#repoRef")]
                        DefsRepoRef(Box<lexicon::com::atproto::admin::defs::RepoRef>),
                        #[serde(alias = "com.atproto.repo.strongRef")]
                        StrongRef(Box<lexicon::com::atproto::repo::strong_ref::Main>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub subject: MainOutputsubjectType,
                        pub id: i64,
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                        pub created_at: String,
                        pub reported_by: String,
                        pub reason: Option<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(tag = "$type")]
                    pub enum MainInputsubjectType {
                        #[serde(alias = "com.atproto.admin.defs#repoRef")]
                        DefsRepoRef(Box<lexicon::com::atproto::admin::defs::RepoRef>),
                        #[serde(alias = "com.atproto.repo.strongRef")]
                        StrongRef(Box<lexicon::com::atproto::repo::strong_ref::Main>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                        pub reason: Option<String>,
                        pub subject: MainInputsubjectType,
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
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    pub struct ReasonViolation;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    pub struct ReasonMisleading;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    pub struct ReasonSpam;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    pub struct ReasonOther;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    pub struct ReasonSexual;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    pub struct ReasonRude;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(tag = "$type")]
                    pub enum ReasonType {
                        #[serde(alias = "com.atproto.moderation.defs#reasonSpam")]
                        DefsReasonSpam(Box<lexicon::com::atproto::moderation::defs::ReasonSpam>),
                        #[serde(alias = "com.atproto.moderation.defs#reasonViolation")]
                        DefsReasonViolation(
                            Box<lexicon::com::atproto::moderation::defs::ReasonViolation>,
                        ),
                        #[serde(alias = "com.atproto.moderation.defs#reasonMisleading")]
                        DefsReasonMisleading(
                            Box<lexicon::com::atproto::moderation::defs::ReasonMisleading>,
                        ),
                        #[serde(alias = "com.atproto.moderation.defs#reasonSexual")]
                        DefsReasonSexual(
                            Box<lexicon::com::atproto::moderation::defs::ReasonSexual>,
                        ),
                        #[serde(alias = "com.atproto.moderation.defs#reasonRude")]
                        DefsReasonRude(Box<lexicon::com::atproto::moderation::defs::ReasonRude>),
                        #[serde(alias = "com.atproto.moderation.defs#reasonOther")]
                        DefsReasonOther(Box<lexicon::com::atproto::moderation::defs::ReasonOther>),
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
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
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
                        pub collection: String,
                        pub rkey: Option<String>,
                        pub value: ::serde_json::Value,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Update {
                        pub value: ::serde_json::Value,
                        pub rkey: String,
                        pub collection: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(tag = "$type")]
                    pub enum WritesType {
                        #[serde(alias = "#create")]
                        Create(Box<Create>),
                        #[serde(alias = "#update")]
                        Update(Box<Update>),
                        #[serde(alias = "#delete")]
                        Delete(Box<Delete>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub swap_commit: Option<String>,
                        pub writes: Vec<WritesType>,
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
                        pub validate: Option<bool>,
                        pub collection: String,
                        pub rkey: Option<String>,
                        pub repo: String,
                        pub record: ::serde_json::Value,
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
                        pub repo: String,
                        pub swap_record: Option<String>,
                        pub rkey: String,
                        pub swap_commit: Option<String>,
                        pub collection: String,
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
                        pub handle: String,
                        pub collections: Vec<String>,
                        pub didDoc: ::serde_json::Value,
                        pub handle_is_correct: bool,
                    }
                    #[doc = "Description: \"Get information about the repo, including the list of collections.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub fn main(token: &String, repo: String) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.repo.describeRepo".to_string(),
                        )
                        .param("repo".to_string(), repo.to_string())
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
                        pub cid: Option<String>,
                        pub uri: String,
                        pub value: ::serde_json::Value,
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
                        .param("repo".to_string(), repo.to_string())
                        .param("collection".to_string(), collection.to_string())
                        .param("rkey".to_string(), rkey.to_string())
                        .param("cid".to_string(), cid.to_string())
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
                    pub struct Record {
                        pub cid: String,
                        pub value: ::serde_json::Value,
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
                        .param("limit".to_string(), limit.to_string())
                        .param("cursor".to_string(), cursor.to_string())
                        .param("rkeyEnd".to_string(), rkey_end.to_string())
                        .param("rkeyStart".to_string(), rkey_start.to_string())
                        .param("collection".to_string(), collection.to_string())
                        .param("reverse".to_string(), reverse.to_string())
                        .param("repo".to_string(), repo.to_string())
                        .token(token);
                        query.execute::<MainOutput>()
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
                        pub cid: String,
                        pub uri: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub repo: String,
                        pub collection: String,
                        pub validate: Option<bool>,
                        pub record: ::serde_json::Value,
                        pub swap_commit: Option<String>,
                        pub swap_record: Option<String>,
                        pub rkey: String,
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
                        pub access_jwt: String,
                        pub handle: String,
                        pub refresh_jwt: String,
                        pub did: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub handle: String,
                        pub password: String,
                        pub recovery_key: Option<String>,
                        pub email: String,
                        pub invite_code: Option<String>,
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
                        pub created_at: String,
                        pub password: String,
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
                        pub email: Option<String>,
                        pub access_jwt: String,
                        pub refresh_jwt: String,
                        pub handle: String,
                        pub did: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub password: String,
                        pub identifier: String,
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
                        pub created_by: String,
                        pub available: i64,
                        pub created_at: String,
                        pub code: String,
                        pub disabled: bool,
                        pub uses: Vec<InviteCodeUse>,
                        pub for_account: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct InviteCodeUse {
                        pub used_by: String,
                        pub used_at: String,
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
                        pub token: String,
                        pub did: String,
                        pub password: String,
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
                        pub available_user_domains: Vec<String>,
                        pub invite_code_required: Option<bool>,
                        pub links: Option<Links>,
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
                        .param("createAvailable".to_string(), create_available.to_string())
                        .param("includeUsed".to_string(), include_used.to_string())
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
                        pub email: Option<String>,
                        pub handle: String,
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
                        pub refresh_jwt: String,
                        pub access_jwt: String,
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
                        .param("did".to_string(), did.to_string())
                        .param("cid".to_string(), cid.to_string())
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
                        .param("did".to_string(), did.to_string())
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
                        .param("commit".to_string(), commit.to_string())
                        .param("did".to_string(), did.to_string())
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
                        .param("did".to_string(), did.to_string())
                        .param("latest".to_string(), latest.to_string())
                        .param("earliest".to_string(), earliest.to_string())
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
                        .param("did".to_string(), did.to_string())
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
                        .param("rkey".to_string(), rkey.to_string())
                        .param("commit".to_string(), commit.to_string())
                        .param("collection".to_string(), collection.to_string())
                        .param("did".to_string(), did.to_string())
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
                        .param("did".to_string(), did.to_string())
                        .param("earliest".to_string(), earliest.to_string())
                        .param("latest".to_string(), latest.to_string())
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
                        .param("did".to_string(), did.to_string())
                        .param("latest".to_string(), latest.to_string())
                        .param("earliest".to_string(), earliest.to_string())
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
                        .param("limit".to_string(), limit.to_string())
                        .param("cursor".to_string(), cursor.to_string())
                        .token(token);
                        query.execute::<MainOutput>()
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Repo {
                        pub did: String,
                        pub head: String,
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
                        .param("hostname".to_string(), hostname.to_string())
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
                        .param("hostname".to_string(), hostname.to_string())
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
                    #[serde(tag = "$type")]
                    pub enum Name {
                        OutdatedCursor,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Info {
                        pub message: Option<String>,
                        pub name: Name,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(tag = "$type")]
                    pub enum Action {
                        Create,
                        Update,
                        Delete,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RepoOp {
                        pub path: String,
                        pub action: Action,
                        pub cid: Option<::serde_json::Value>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Commit {
                        pub seq: i64,
                        pub repo: String,
                        pub time: String,
                        pub ops: Vec<RepoOp>,
                        pub blocks: ::serde_json::Value,
                        pub commit: ::serde_json::Value,
                        pub rebase: bool,
                        pub blobs: Vec<Unimplemented>,
                        pub too_big: bool,
                        pub prev: Option<::serde_json::Value>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Handle {
                        pub handle: String,
                        pub seq: i64,
                        pub did: String,
                        pub time: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Tombstone {
                        pub did: String,
                        pub time: String,
                        pub seq: i64,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Migrate {
                        pub migrate_to: Option<String>,
                        pub time: String,
                        pub seq: i64,
                        pub did: String,
                    }
                }
            }
        }
    }
}

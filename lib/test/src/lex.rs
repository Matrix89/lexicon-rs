struct Xrpc {}
pub mod lexicon {
    pub mod did {
        pub type Did = String;
    }
    pub mod cid {
        pub type Cid = String;
    }
    pub mod handle {
        pub type Handle = String;
    }
    pub mod at_uri {
        pub type AtUri = String;
    }
    pub mod at_identifier {
        pub type AtIdentifier = String;
    }
    pub mod nsid {
        pub type Nsid = String;
    }
    pub mod url {
        pub type Url = String;
    }
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
                pub mod get_profile {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        actor: String,
                    ) -> Result<lexicon::app::bsky::actor::defs::ProfileViewDetailed, XrpcError>
                    {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.actor.getProfile".to_string(),
                        );
                        req.param("actor".to_string(), actor.to_string());
                        req.token(token);
                        req.execute::<lexicon::app::bsky::actor::defs::ProfileViewDetailed>()
                            .await
                    }
                }
                pub mod get_profiles {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub profiles: Vec<lexicon::app::bsky::actor::defs::ProfileViewDetailed>,
                    }
                    pub async fn main(
                        token: &String,
                        actors: Vec<String>,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.actor.getProfiles".to_string(),
                        );
                        for ident in actors {
                            req.param("actors".to_string(), ident.to_string());
                        }
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod get_suggestions {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub actors: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    pub async fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.actor.getSuggestions".to_string(),
                        );
                        req.param("cursor".to_string(), cursor.to_string());
                        req.param("limit".to_string(), limit.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod profile {
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
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub description: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub avatar: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub banner: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub display_name: Option<String>,
                    }
                }
                pub mod search_actors {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub actors: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    pub async fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                        term: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.actor.searchActors".to_string(),
                        );
                        req.param("term".to_string(), term.to_string());
                        req.param("cursor".to_string(), cursor.to_string());
                        req.param("limit".to_string(), limit.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod search_actors_typeahead {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub actors: Vec<lexicon::app::bsky::actor::defs::ProfileViewBasic>,
                    }
                    pub async fn main(
                        token: &String,
                        limit: i64,
                        term: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.actor.searchActorsTypeahead"
                                .to_string(),
                        );
                        req.param("limit".to_string(), limit.to_string());
                        req.param("term".to_string(), term.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
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
                    pub struct SavedFeedsPref {
                        pub pinned: Vec<String>,
                        pub saved: Vec<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ProfileView {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub viewer: Option<ViewerState>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub description: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub indexed_at: Option<chrono::DateTime<chrono::Utc>>,
                        pub handle: lexicon::handle::Handle,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub display_name: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub avatar: Option<String>,
                        pub did: lexicon::did::Did,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(tag = "$type")]
                    pub enum Visibility {
                        Show,
                        Warn,
                        Hide,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ContentLabelPref {
                        pub visibility: Visibility,
                        pub label: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct AdultContentPref {
                        pub enabled: bool,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ProfileViewBasic {
                        pub did: lexicon::did::Did,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub avatar: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub viewer: Option<ViewerState>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub display_name: Option<String>,
                        pub handle: lexicon::handle::Handle,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ProfileViewDetailed {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub followers_count: Option<i64>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub posts_count: Option<i64>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub description: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub display_name: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub viewer: Option<ViewerState>,
                        pub handle: lexicon::handle::Handle,
                        pub did: lexicon::did::Did,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub avatar: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub follows_count: Option<i64>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub indexed_at: Option<chrono::DateTime<chrono::Utc>>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub banner: Option<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ViewerState {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub muted: Option<bool>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub muted_by_list: Option<lexicon::app::bsky::graph::defs::ListViewBasic>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub followed_by: Option<lexicon::at_uri::AtUri>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub blocked_by: Option<bool>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub following: Option<lexicon::at_uri::AtUri>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub blocking: Option<lexicon::at_uri::AtUri>,
                    }
                }
                pub mod get_preferences {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub preferences: lexicon::app::bsky::actor::defs::Preferences,
                    }
                    pub async fn main(token: &String) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.actor.getPreferences".to_string(),
                        );
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod put_preferences {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub preferences: lexicon::app::bsky::actor::defs::Preferences,
                    }
                    pub async fn main(token: &String, input: MainInput) -> Result<(), XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/app.bsky.actor.putPreferences".to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<()>().await
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
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub thumb: Option<String>,
                        pub uri: lexicon::url::Url,
                        pub description: String,
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
                        pub uri: lexicon::url::Url,
                        pub title: String,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub thumb: Option<String>,
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
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Image {
                        pub alt: String,
                        pub image: String,
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
                        pub record: lexicon::app::bsky::embed::record::Main,
                        pub media: MainmediaType,
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
                        pub uri: lexicon::at_uri::AtUri,
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
                        #[serde(alias = "app.bsky.feed.defs#generatorView")]
                        DefsGeneratorView(Box<lexicon::app::bsky::feed::defs::GeneratorView>),
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
                    #[serde(rename_all = "camelCase")]
                    pub struct ViewNotFound {
                        pub uri: lexicon::at_uri::AtUri,
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
                        pub indexed_at: chrono::DateTime<chrono::Utc>,
                        pub cid: lexicon::cid::Cid,
                        pub author: lexicon::app::bsky::actor::defs::ProfileViewBasic,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        pub value: ::serde_json::Value,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub embeds: Option<Vec<EmbedsType>>,
                        pub uri: lexicon::at_uri::AtUri,
                    }
                }
            }
            pub mod feed {
                #[allow(unused_imports)]
                use super::lexicon;
                pub mod get_author_feed {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub feed: Vec<lexicon::app::bsky::feed::defs::FeedViewPost>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    pub async fn main(
                        token: &String,
                        actor: String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.feed.getAuthorFeed".to_string(),
                        );
                        req.param("actor".to_string(), actor.to_string());
                        req.param("limit".to_string(), limit.to_string());
                        req.param("cursor".to_string(), cursor.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
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
                    pub struct Like {
                        pub created_at: chrono::DateTime<chrono::Utc>,
                        pub indexed_at: chrono::DateTime<chrono::Utc>,
                        pub actor: lexicon::app::bsky::actor::defs::ProfileView,
                    }
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cid: Option<lexicon::cid::Cid>,
                        pub likes: Vec<Like>,
                        pub uri: lexicon::at_uri::AtUri,
                    }
                    pub async fn main(
                        token: &String,
                        cid: String,
                        cursor: String,
                        limit: i64,
                        uri: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.feed.getLikes".to_string(),
                        );
                        req.param("uri".to_string(), uri.to_string());
                        req.param("cid".to_string(), cid.to_string());
                        req.param("limit".to_string(), limit.to_string());
                        req.param("cursor".to_string(), cursor.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod get_post_thread {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
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
                    pub async fn main(
                        token: &String,
                        depth: i64,
                        uri: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.feed.getPostThread".to_string(),
                        );
                        req.param("uri".to_string(), uri.to_string());
                        req.param("depth".to_string(), depth.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod get_posts {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub posts: Vec<lexicon::app::bsky::feed::defs::PostView>,
                    }
                    pub async fn main(
                        token: &String,
                        uris: Vec<String>,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.feed.getPosts".to_string(),
                        );
                        for ident in uris {
                            req.param("uris".to_string(), ident.to_string());
                        }
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod get_reposted_by {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub uri: lexicon::at_uri::AtUri,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cid: Option<lexicon::cid::Cid>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                        pub reposted_by: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                    }
                    pub async fn main(
                        token: &String,
                        cid: String,
                        cursor: String,
                        limit: i64,
                        uri: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.feed.getRepostedBy".to_string(),
                        );
                        req.param("cursor".to_string(), cursor.to_string());
                        req.param("cid".to_string(), cid.to_string());
                        req.param("limit".to_string(), limit.to_string());
                        req.param("uri".to_string(), uri.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod get_timeline {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                        pub feed: Vec<lexicon::app::bsky::feed::defs::FeedViewPost>,
                    }
                    pub async fn main(
                        token: &String,
                        algorithm: String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.feed.getTimeline".to_string(),
                        );
                        req.param("algorithm".to_string(), algorithm.to_string());
                        req.param("cursor".to_string(), cursor.to_string());
                        req.param("limit".to_string(), limit.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod like {
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
                        pub subject: lexicon::com::atproto::repo::strong_ref::Main,
                        pub created_at: chrono::DateTime<chrono::Utc>,
                    }
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
                    pub struct Entity {
                        pub index: TextSlice,
                        pub value: String,
                        pub r#type: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(tag = "$type")]
                    pub enum MainembedType {
                        #[serde(alias = "app.bsky.embed.images")]
                        Images(Box<lexicon::app::bsky::embed::images::Main>),
                        #[serde(alias = "app.bsky.embed.external")]
                        External(Box<lexicon::app::bsky::embed::external::Main>),
                        #[serde(alias = "app.bsky.embed.record")]
                        Record(Box<lexicon::app::bsky::embed::record::Main>),
                        #[serde(alias = "app.bsky.embed.recordWithMedia")]
                        RecordWithMedia(Box<lexicon::app::bsky::embed::record_with_media::Main>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Main {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub entities: Option<Vec<Entity>>,
                        pub created_at: chrono::DateTime<chrono::Utc>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub embed: Option<MainembedType>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub facets: Option<Vec<lexicon::app::bsky::richtext::facet::Main>>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub reply: Option<ReplyRef>,
                        pub text: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct TextSlice {
                        pub end: i64,
                        pub start: i64,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ReplyRef {
                        pub root: lexicon::com::atproto::repo::strong_ref::Main,
                        pub parent: lexicon::com::atproto::repo::strong_ref::Main,
                    }
                }
                pub mod repost {
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
                        pub created_at: chrono::DateTime<chrono::Utc>,
                        pub subject: lexicon::com::atproto::repo::strong_ref::Main,
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
                    pub struct ReasonRepost {
                        pub indexed_at: chrono::DateTime<chrono::Utc>,
                        pub by: lexicon::app::bsky::actor::defs::ProfileViewBasic,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(tag = "$type")]
                    pub enum ReplyRefparentType {
                        #[serde(alias = "app.bsky.feed.defs#postView")]
                        PostView(Box<PostView>),
                        #[serde(alias = "app.bsky.feed.defs#notFoundPost")]
                        NotFoundPost(Box<NotFoundPost>),
                        #[serde(alias = "app.bsky.feed.defs#blockedPost")]
                        BlockedPost(Box<BlockedPost>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(tag = "$type")]
                    pub enum ReplyRefrootType {
                        #[serde(alias = "app.bsky.feed.defs#postView")]
                        PostView(Box<PostView>),
                        #[serde(alias = "app.bsky.feed.defs#notFoundPost")]
                        NotFoundPost(Box<NotFoundPost>),
                        #[serde(alias = "app.bsky.feed.defs#blockedPost")]
                        BlockedPost(Box<BlockedPost>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ReplyRef {
                        pub parent: ReplyRefparentType,
                        pub root: ReplyRefrootType,
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
                    #[serde(rename_all = "camelCase")]
                    pub struct ThreadViewPost {
                        pub post: PostView,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub replies: Option<Vec<RepliesType>>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub parent: Option<ThreadViewPostparentType>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct GeneratorViewerState {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub like: Option<lexicon::at_uri::AtUri>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(tag = "$type")]
                    pub enum SkeletonFeedPostreasonType {
                        #[serde(alias = "app.bsky.feed.defs#skeletonReasonRepost")]
                        SkeletonReasonRepost(Box<SkeletonReasonRepost>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct SkeletonFeedPost {
                        pub post: lexicon::at_uri::AtUri,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub reason: Option<SkeletonFeedPostreasonType>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct NotFoundPost {
                        pub uri: lexicon::at_uri::AtUri,
                        pub not_found: bool,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct SkeletonReasonRepost {
                        pub repost: lexicon::at_uri::AtUri,
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
                        pub record: ::serde_json::Value,
                        pub uri: lexicon::at_uri::AtUri,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub repost_count: Option<i64>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub viewer: Option<ViewerState>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub embed: Option<PostViewembedType>,
                        pub cid: lexicon::cid::Cid,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub reply_count: Option<i64>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub like_count: Option<i64>,
                        pub indexed_at: chrono::DateTime<chrono::Utc>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        pub author: lexicon::app::bsky::actor::defs::ProfileViewBasic,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct GeneratorView {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub viewer: Option<GeneratorViewerState>,
                        pub uri: lexicon::at_uri::AtUri,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub avatar: Option<String>,
                        pub display_name: String,
                        pub indexed_at: chrono::DateTime<chrono::Utc>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub did: Option<lexicon::did::Did>,
                        pub cid: lexicon::cid::Cid,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub description: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub description_facets:
                            Option<Vec<lexicon::app::bsky::richtext::facet::Main>>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub like_count: Option<i64>,
                        pub creator: lexicon::app::bsky::actor::defs::ProfileView,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ViewerState {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub like: Option<lexicon::at_uri::AtUri>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub repost: Option<lexicon::at_uri::AtUri>,
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
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub reason: Option<FeedViewPostreasonType>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub reply: Option<ReplyRef>,
                        pub post: PostView,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct BlockedPost {
                        pub blocked: bool,
                        pub uri: lexicon::at_uri::AtUri,
                    }
                }
                pub mod describe_feed_generator {
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
                    pub struct Feed {
                        pub uri: lexicon::at_uri::AtUri,
                    }
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub links: Option<Links>,
                        pub feeds: Vec<Feed>,
                        pub did: lexicon::did::Did,
                    }
                    pub async fn main(token: &String) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.feed.describeFeedGenerator"
                                .to_string(),
                        );
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Links {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub terms_of_service: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub privacy_policy: Option<String>,
                    }
                }
                pub mod generator {
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
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub description: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub description_facets:
                            Option<Vec<lexicon::app::bsky::richtext::facet::Main>>,
                        pub created_at: chrono::DateTime<chrono::Utc>,
                        pub did: lexicon::did::Did,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub avatar: Option<String>,
                        pub display_name: String,
                    }
                }
                pub mod get_actor_feeds {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                        pub feeds: Vec<lexicon::app::bsky::feed::defs::GeneratorView>,
                    }
                    pub async fn main(
                        token: &String,
                        actor: String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.feed.getActorFeeds".to_string(),
                        );
                        req.param("limit".to_string(), limit.to_string());
                        req.param("cursor".to_string(), cursor.to_string());
                        req.param("actor".to_string(), actor.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod get_feed {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                        pub feed: Vec<lexicon::app::bsky::feed::defs::FeedViewPost>,
                    }
                    pub async fn main(
                        token: &String,
                        cursor: String,
                        feed: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.feed.getFeed".to_string(),
                        );
                        req.param("feed".to_string(), feed.to_string());
                        req.param("cursor".to_string(), cursor.to_string());
                        req.param("limit".to_string(), limit.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod get_feed_generator {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub is_online: bool,
                        pub is_valid: bool,
                        pub view: lexicon::app::bsky::feed::defs::GeneratorView,
                    }
                    pub async fn main(
                        token: &String,
                        feed: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.feed.getFeedGenerator".to_string(),
                        );
                        req.param("feed".to_string(), feed.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod get_feed_generators {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub feeds: Vec<lexicon::app::bsky::feed::defs::GeneratorView>,
                    }
                    pub async fn main(
                        token: &String,
                        feeds: Vec<String>,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.feed.getFeedGenerators".to_string(),
                        );
                        for ident in feeds {
                            req.param("feeds".to_string(), ident.to_string());
                        }
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod get_feed_skeleton {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub feed: Vec<lexicon::app::bsky::feed::defs::SkeletonFeedPost>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    pub async fn main(
                        token: &String,
                        cursor: String,
                        feed: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.feed.getFeedSkeleton".to_string(),
                        );
                        req.param("limit".to_string(), limit.to_string());
                        req.param("feed".to_string(), feed.to_string());
                        req.param("cursor".to_string(), cursor.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
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
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Main {
                        pub subject: lexicon::did::Did,
                        pub created_at: chrono::DateTime<chrono::Utc>,
                    }
                }
                pub mod follow {
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
                        pub subject: lexicon::did::Did,
                        pub created_at: chrono::DateTime<chrono::Utc>,
                    }
                }
                pub mod get_blocks {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub blocks: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    pub async fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.graph.getBlocks".to_string(),
                        );
                        req.param("limit".to_string(), limit.to_string());
                        req.param("cursor".to_string(), cursor.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod get_followers {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub subject: lexicon::app::bsky::actor::defs::ProfileView,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                        pub followers: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                    }
                    pub async fn main(
                        token: &String,
                        actor: String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.graph.getFollowers".to_string(),
                        );
                        req.param("cursor".to_string(), cursor.to_string());
                        req.param("limit".to_string(), limit.to_string());
                        req.param("actor".to_string(), actor.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod get_follows {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                        pub follows: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                        pub subject: lexicon::app::bsky::actor::defs::ProfileView,
                    }
                    pub async fn main(
                        token: &String,
                        actor: String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.graph.getFollows".to_string(),
                        );
                        req.param("limit".to_string(), limit.to_string());
                        req.param("actor".to_string(), actor.to_string());
                        req.param("cursor".to_string(), cursor.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod get_mutes {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub mutes: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    pub async fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.graph.getMutes".to_string(),
                        );
                        req.param("limit".to_string(), limit.to_string());
                        req.param("cursor".to_string(), cursor.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod mute_actor {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub actor: lexicon::at_identifier::AtIdentifier,
                    }
                    pub async fn main(token: &String, input: MainInput) -> Result<(), XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/app.bsky.graph.muteActor".to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<()>().await
                    }
                }
                pub mod unmute_actor {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub actor: lexicon::at_identifier::AtIdentifier,
                    }
                    pub async fn main(token: &String, input: MainInput) -> Result<(), XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/app.bsky.graph.unmuteActor".to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<()>().await
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
                    pub enum ListPurpose {
                        #[serde(alias = "app.bsky.graph.defs#modlist")]
                        DefsModlist(Box<lexicon::app::bsky::graph::defs::Modlist>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ListView {
                        pub uri: lexicon::at_uri::AtUri,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub description: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub description_facets:
                            Option<Vec<lexicon::app::bsky::richtext::facet::Main>>,
                        pub purpose: ListPurpose,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub avatar: Option<String>,
                        pub name: String,
                        pub indexed_at: chrono::DateTime<chrono::Utc>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub viewer: Option<ListViewerState>,
                        pub creator: lexicon::app::bsky::actor::defs::ProfileView,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ListViewBasic {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub viewer: Option<ListViewerState>,
                        pub name: String,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub indexed_at: Option<chrono::DateTime<chrono::Utc>>,
                        pub purpose: ListPurpose,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub avatar: Option<String>,
                        pub uri: lexicon::at_uri::AtUri,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ListItemView {
                        pub subject: lexicon::app::bsky::actor::defs::ProfileView,
                    }
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    #[doc = "A list of actors to apply an aggregate moderation action (mute/block) on"]
                    pub struct Modlist;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ListViewerState {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub muted: Option<bool>,
                    }
                }
                pub mod get_list {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                        pub items: Vec<lexicon::app::bsky::graph::defs::ListItemView>,
                        pub list: lexicon::app::bsky::graph::defs::ListView,
                    }
                    pub async fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                        list: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.graph.getList".to_string(),
                        );
                        req.param("list".to_string(), list.to_string());
                        req.param("limit".to_string(), limit.to_string());
                        req.param("cursor".to_string(), cursor.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod get_list_mutes {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub lists: Vec<lexicon::app::bsky::graph::defs::ListView>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    pub async fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.graph.getListMutes".to_string(),
                        );
                        req.param("limit".to_string(), limit.to_string());
                        req.param("cursor".to_string(), cursor.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod get_lists {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub lists: Vec<lexicon::app::bsky::graph::defs::ListView>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    pub async fn main(
                        token: &String,
                        actor: String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.graph.getLists".to_string(),
                        );
                        req.param("cursor".to_string(), cursor.to_string());
                        req.param("limit".to_string(), limit.to_string());
                        req.param("actor".to_string(), actor.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod list {
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
                        pub purpose: lexicon::app::bsky::graph::defs::ListPurpose,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub description_facets:
                            Option<Vec<lexicon::app::bsky::richtext::facet::Main>>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub description: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub avatar: Option<String>,
                        pub name: String,
                        pub created_at: chrono::DateTime<chrono::Utc>,
                    }
                }
                pub mod listitem {
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
                        pub created_at: chrono::DateTime<chrono::Utc>,
                        pub list: lexicon::at_uri::AtUri,
                        pub subject: lexicon::did::Did,
                    }
                }
                pub mod mute_actor_list {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub list: lexicon::at_uri::AtUri,
                    }
                    pub async fn main(token: &String, input: MainInput) -> Result<(), XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/app.bsky.graph.muteActorList".to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<()>().await
                    }
                }
                pub mod unmute_actor_list {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub list: lexicon::at_uri::AtUri,
                    }
                    pub async fn main(token: &String, input: MainInput) -> Result<(), XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/app.bsky.graph.unmuteActorList".to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<()>().await
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub count: i64,
                    }
                    pub async fn main(
                        token: &String,
                        seen_at: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.notification.getUnreadCount"
                                .to_string(),
                        );
                        req.param("seenAt".to_string(), seen_at.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
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
                        pub indexed_at: chrono::DateTime<chrono::Utc>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub reason_subject: Option<lexicon::at_uri::AtUri>,
                        pub is_read: bool,
                        pub author: lexicon::app::bsky::actor::defs::ProfileView,
                        pub reason: Reason,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        pub uri: lexicon::at_uri::AtUri,
                        pub cid: lexicon::cid::Cid,
                        pub record: ::serde_json::Value,
                    }
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub notifications: Vec<Notification>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    pub async fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                        seen_at: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.notification.listNotifications"
                                .to_string(),
                        );
                        req.param("limit".to_string(), limit.to_string());
                        req.param("cursor".to_string(), cursor.to_string());
                        req.param("seenAt".to_string(), seen_at.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod update_seen {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub seen_at: chrono::DateTime<chrono::Utc>,
                    }
                    pub async fn main(token: &String, input: MainInput) -> Result<(), XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/app.bsky.notification.updateSeen".to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<()>().await
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
                        pub did: lexicon::did::Did,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ByteSlice {
                        pub byte_start: i64,
                        pub byte_end: i64,
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
                    pub struct Link {
                        pub uri: lexicon::url::Url,
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub feed: Vec<lexicon::app::bsky::feed::defs::FeedViewPost>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    pub async fn main(
                        token: &String,
                        cursor: String,
                        include_nsfw: bool,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.unspecced.getPopular".to_string(),
                        );
                        req.param("includeNsfw".to_string(), include_nsfw.to_string());
                        req.param("limit".to_string(), limit.to_string());
                        req.param("cursor".to_string(), cursor.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod get_popular_feed_generators {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub feeds: Vec<lexicon::app::bsky::feed::defs::GeneratorView>,
                    }
                    pub async fn main(token: &String) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.unspecced.getPopularFeedGenerators"
                                .to_string(),
                        );
                        req.token(token);
                        req.execute::<MainOutput>().await
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
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub accounts: Option<Vec<String>>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub codes: Option<Vec<String>>,
                    }
                    pub async fn main(token: &String, input: MainInput) -> Result<(), XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.admin.disableInviteCodes"
                                .to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<()>().await
                    }
                }
                pub mod get_invite_codes {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub codes: Vec<lexicon::com::atproto::server::defs::InviteCode>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    pub async fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                        sort: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.admin.getInviteCodes".to_string(),
                        );
                        req.param("sort".to_string(), sort.to_string());
                        req.param("limit".to_string(), limit.to_string());
                        req.param("cursor".to_string(), cursor.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod get_moderation_action {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        id: i64,
                    ) -> Result<lexicon::com::atproto::admin::defs::ActionViewDetail, XrpcError>
                    {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.admin.getModerationAction"
                                .to_string(),
                        );
                        req.param("id".to_string(), id.to_string());
                        req.token(token);
                        req.execute::<lexicon::com::atproto::admin::defs::ActionViewDetail>()
                            .await
                    }
                }
                pub mod get_moderation_actions {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                        pub actions: Vec<lexicon::com::atproto::admin::defs::ActionView>,
                    }
                    pub async fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                        subject: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.admin.getModerationActions"
                                .to_string(),
                        );
                        req.param("subject".to_string(), subject.to_string());
                        req.param("limit".to_string(), limit.to_string());
                        req.param("cursor".to_string(), cursor.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod get_moderation_report {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        id: i64,
                    ) -> Result<lexicon::com::atproto::admin::defs::ReportViewDetail, XrpcError>
                    {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.admin.getModerationReport"
                                .to_string(),
                        );
                        req.param("id".to_string(), id.to_string());
                        req.token(token);
                        req.execute::<lexicon::com::atproto::admin::defs::ReportViewDetail>()
                            .await
                    }
                }
                pub mod resolve_moderation_reports {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub action_id: i64,
                        pub report_ids: Vec<i64>,
                        pub created_by: lexicon::did::Did,
                    }
                    pub async fn main(
                        token: &String,
                        input: MainInput,
                    ) -> Result<lexicon::com::atproto::admin::defs::ActionView, XrpcError>
                    {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.admin.resolveModerationReports"
                                .to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<lexicon::com::atproto::admin::defs::ActionView>()
                            .await
                    }
                }
                pub mod reverse_moderation_action {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub reason: String,
                        pub id: i64,
                        pub created_by: lexicon::did::Did,
                    }
                    pub async fn main(
                        token: &String,
                        input: MainInput,
                    ) -> Result<lexicon::com::atproto::admin::defs::ActionView, XrpcError>
                    {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.admin.reverseModerationAction"
                                .to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<lexicon::com::atproto::admin::defs::ActionView>()
                            .await
                    }
                }
                pub mod search_repos {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                        pub repos: Vec<lexicon::com::atproto::admin::defs::RepoView>,
                    }
                    pub async fn main(
                        token: &String,
                        cursor: String,
                        invited_by: String,
                        limit: i64,
                        term: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.admin.searchRepos".to_string(),
                        );
                        req.param("invitedBy".to_string(), invited_by.to_string());
                        req.param("limit".to_string(), limit.to_string());
                        req.param("cursor".to_string(), cursor.to_string());
                        req.param("term".to_string(), term.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod take_moderation_action {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
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
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub subject_blob_cids: Option<Vec<String>>,
                        pub action: Action,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub create_label_vals: Option<Vec<String>>,
                        pub reason: String,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub negate_label_vals: Option<Vec<String>>,
                        pub created_by: lexicon::did::Did,
                        pub subject: MainInputsubjectType,
                    }
                    pub async fn main(
                        token: &String,
                        input: MainInput,
                    ) -> Result<lexicon::com::atproto::admin::defs::ActionView, XrpcError>
                    {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.admin.takeModerationAction"
                                .to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<lexicon::com::atproto::admin::defs::ActionView>()
                            .await
                    }
                }
                pub mod update_account_email {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub email: String,
                        pub account: lexicon::at_identifier::AtIdentifier,
                    }
                    pub async fn main(token: &String, input: MainInput) -> Result<(), XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.admin.updateAccountEmail"
                                .to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<()>().await
                    }
                }
                pub mod update_account_handle {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub did: lexicon::did::Did,
                        pub handle: lexicon::handle::Handle,
                    }
                    pub async fn main(token: &String, input: MainInput) -> Result<(), XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.admin.updateAccountHandle"
                                .to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<()>().await
                    }
                }
                pub mod disable_account_invites {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub account: lexicon::did::Did,
                    }
                    pub async fn main(token: &String, input: MainInput) -> Result<(), XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.admin.disableAccountInvites"
                                .to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<()>().await
                    }
                }
                pub mod enable_account_invites {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub account: lexicon::did::Did,
                    }
                    pub async fn main(token: &String, input: MainInput) -> Result<(), XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.admin.enableAccountInvites"
                                .to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<()>().await
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
                    pub struct RecordViewDetail {
                        pub cid: lexicon::cid::Cid,
                        pub indexed_at: chrono::DateTime<chrono::Utc>,
                        pub moderation: ModerationDetail,
                        pub uri: lexicon::at_uri::AtUri,
                        pub value: ::serde_json::Value,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        pub repo: RepoView,
                        pub blobs: Vec<BlobView>,
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
                    #[serde(rename_all = "camelCase")]
                    pub struct RepoView {
                        pub indexed_at: chrono::DateTime<chrono::Utc>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub email: Option<String>,
                        pub moderation: Moderation,
                        pub handle: lexicon::handle::Handle,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub invited_by: Option<lexicon::com::atproto::server::defs::InviteCode>,
                        pub did: lexicon::did::Did,
                        pub related_records: Vec<Unimplemented>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub invites_disabled: Option<bool>,
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
                        pub mime_type: String,
                        pub cid: lexicon::cid::Cid,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub moderation: Option<Moderation>,
                        pub size: i64,
                        pub created_at: chrono::DateTime<chrono::Utc>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub details: Option<BlobViewdetailsType>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RepoViewDetail {
                        pub did: lexicon::did::Did,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub invites: Option<Vec<lexicon::com::atproto::server::defs::InviteCode>>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub email: Option<String>,
                        pub handle: lexicon::handle::Handle,
                        pub moderation: ModerationDetail,
                        pub related_records: Vec<Unimplemented>,
                        pub indexed_at: chrono::DateTime<chrono::Utc>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub invited_by: Option<lexicon::com::atproto::server::defs::InviteCode>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub invites_disabled: Option<bool>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Moderation {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub current_action: Option<ActionViewCurrent>,
                    }
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    #[doc = "Moderation action type: Takedown. Indicates that content should not be served by the PDS."]
                    pub struct Takedown;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(tag = "$type")]
                    pub enum ReportViewDetailsubjectType {
                        #[serde(alias = "com.atproto.admin.defs#repoView")]
                        RepoView(Box<RepoView>),
                        #[serde(alias = "com.atproto.admin.defs#repoViewNotFound")]
                        RepoViewNotFound(Box<RepoViewNotFound>),
                        #[serde(alias = "com.atproto.admin.defs#recordView")]
                        RecordView(Box<RecordView>),
                        #[serde(alias = "com.atproto.admin.defs#recordViewNotFound")]
                        RecordViewNotFound(Box<RecordViewNotFound>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ReportViewDetail {
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub reason: Option<String>,
                        pub subject: ReportViewDetailsubjectType,
                        pub reported_by: lexicon::did::Did,
                        pub created_at: chrono::DateTime<chrono::Utc>,
                        pub resolved_by_actions:
                            Vec<lexicon::com::atproto::admin::defs::ActionView>,
                        pub id: i64,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(tag = "$type")]
                    pub enum ActionViewDetailsubjectType {
                        #[serde(alias = "com.atproto.admin.defs#repoView")]
                        RepoView(Box<RepoView>),
                        #[serde(alias = "com.atproto.admin.defs#repoViewNotFound")]
                        RepoViewNotFound(Box<RepoViewNotFound>),
                        #[serde(alias = "com.atproto.admin.defs#recordView")]
                        RecordView(Box<RecordView>),
                        #[serde(alias = "com.atproto.admin.defs#recordViewNotFound")]
                        RecordViewNotFound(Box<RecordViewNotFound>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ActionViewDetail {
                        pub subject: ActionViewDetailsubjectType,
                        pub created_by: lexicon::did::Did,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub reversal: Option<ActionReversal>,
                        pub id: i64,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub create_label_vals: Option<Vec<String>>,
                        pub subject_blobs: Vec<BlobView>,
                        pub created_at: chrono::DateTime<chrono::Utc>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub negate_label_vals: Option<Vec<String>>,
                        pub resolved_reports: Vec<ReportView>,
                        pub reason: String,
                        pub action: ActionType,
                    }
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    #[doc = "Moderation action type: Acknowledge. Indicates that the content was reviewed and not considered to violate PDS rules."]
                    pub struct Acknowledge;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ModerationDetail {
                        pub actions: Vec<ActionView>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub current_action: Option<ActionViewCurrent>,
                        pub reports: Vec<ReportView>,
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
                        pub reported_by: lexicon::did::Did,
                        pub created_at: chrono::DateTime<chrono::Utc>,
                        pub id: i64,
                        pub resolved_by_action_ids: Vec<i64>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub reason: Option<String>,
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                        pub subject: ReportViewsubjectType,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RepoViewNotFound {
                        pub did: lexicon::did::Did,
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
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    #[doc = "Moderation action type: Flag. Indicates that the content was reviewed and considered to violate PDS rules, but may still be served."]
                    pub struct Flag;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    #[doc = "Moderation action type: Escalate. Indicates that the content has been flagged for additional review."]
                    pub struct Escalate;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ActionReversal {
                        pub created_by: lexicon::did::Did,
                        pub created_at: chrono::DateTime<chrono::Utc>,
                        pub reason: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RepoRef {
                        pub did: lexicon::did::Did,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RecordViewNotFound {
                        pub uri: lexicon::at_uri::AtUri,
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
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RecordView {
                        pub value: ::serde_json::Value,
                        pub blob_cids: Vec<String>,
                        pub indexed_at: chrono::DateTime<chrono::Utc>,
                        pub moderation: Moderation,
                        pub repo: RepoView,
                        pub cid: lexicon::cid::Cid,
                        pub uri: lexicon::at_uri::AtUri,
                    }
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
                        pub created_at: chrono::DateTime<chrono::Utc>,
                        pub resolved_report_ids: Vec<i64>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub create_label_vals: Option<Vec<String>>,
                        pub action: ActionType,
                        pub id: i64,
                        pub subject: ActionViewsubjectType,
                        pub subject_blob_cids: Vec<String>,
                        pub reason: String,
                        pub created_by: lexicon::did::Did,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub reversal: Option<ActionReversal>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub negate_label_vals: Option<Vec<String>>,
                    }
                }
                pub mod get_moderation_reports {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub reports: Vec<lexicon::com::atproto::admin::defs::ReportView>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    pub async fn main(
                        token: &String,
                        action_type: String,
                        cursor: String,
                        limit: i64,
                        resolved: bool,
                        subject: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.admin.getModerationReports"
                                .to_string(),
                        );
                        req.param("cursor".to_string(), cursor.to_string());
                        req.param("resolved".to_string(), resolved.to_string());
                        req.param("limit".to_string(), limit.to_string());
                        req.param("subject".to_string(), subject.to_string());
                        req.param("actionType".to_string(), action_type.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod get_record {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        cid: String,
                        uri: String,
                    ) -> Result<lexicon::com::atproto::admin::defs::RecordViewDetail, XrpcError>
                    {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.admin.getRecord".to_string(),
                        );
                        req.param("uri".to_string(), uri.to_string());
                        req.param("cid".to_string(), cid.to_string());
                        req.token(token);
                        req.execute::<lexicon::com::atproto::admin::defs::RecordViewDetail>()
                            .await
                    }
                }
                pub mod get_repo {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        did: String,
                    ) -> Result<lexicon::com::atproto::admin::defs::RepoViewDetail, XrpcError>
                    {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.admin.getRepo".to_string(),
                        );
                        req.param("did".to_string(), did.to_string());
                        req.token(token);
                        req.execute::<lexicon::com::atproto::admin::defs::RepoViewDetail>()
                            .await
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub did: lexicon::did::Did,
                    }
                    #[doc = "Description: \"Provides the DID of a repo.\""]
                    #[doc = "# Arguments"]
                    #[doc = "* `handle` - \"The handle to resolve. If not supplied, will resolve the host's own handle.\""]
                    pub async fn main(
                        token: &String,
                        handle: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.identity.resolveHandle"
                                .to_string(),
                        );
                        req.param("handle".to_string(), handle.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod update_handle {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub handle: lexicon::handle::Handle,
                    }
                    pub async fn main(token: &String, input: MainInput) -> Result<(), XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.identity.updateHandle"
                                .to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<()>().await
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
                        pub val: String,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub neg: Option<bool>,
                        pub cts: chrono::DateTime<chrono::Utc>,
                        pub src: lexicon::did::Did,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cid: Option<lexicon::cid::Cid>,
                        pub uri: lexicon::url::Url,
                    }
                }
                pub mod query_labels {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
                    }
                    #[doc = "Description: \"Find labels relevant to the provided URI patterns.\""]
                    #[doc = "# Arguments"]
                    #[doc = "* `uriPatterns` - \"List of AT URI patterns to match (boolean 'OR'). Each may be a prefix (ending with '*'; will match inclusive of the string leading to '*'), or a full URI\""]
                    #[doc = "* `sources` - \"Optional list of label sources (DIDs) to filter on\""]
                    pub async fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                        sources: Vec<String>,
                        uri_patterns: Vec<String>,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.label.queryLabels".to_string(),
                        );
                        req.param("cursor".to_string(), cursor.to_string());
                        for ident in sources {
                            req.param("sources".to_string(), ident.to_string());
                        }
                        for ident in uri_patterns {
                            req.param("uriPatterns".to_string(), ident.to_string());
                        }
                        req.param("limit".to_string(), limit.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod subscribe_labels {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[doc = "Subscribe to label updates"]
                    use xrpc::error::XrpcError;
                    use xrpc::subscription::XrpcSubscription;
                    pub fn main(token: &String, cursor: i64) {
                        let mut req = XrpcSubscription::new(
                            "wss://bsky.social/xrpc/com.atproto.label.subscribeLabels".to_string(),
                        );
                        req.param("cursor".to_string(), cursor.to_string());
                        req.token(token);
                        req.subscribe();
                    }
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
                        pub name: Name,
                        #[serde(skip_serializing_if = "Option::is_none")]
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
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
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
                        pub created_at: chrono::DateTime<chrono::Utc>,
                        pub id: i64,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub reason: Option<String>,
                        pub subject: MainOutputsubjectType,
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                        pub reported_by: lexicon::did::Did,
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
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub reason: Option<String>,
                        pub subject: MainInputsubjectType,
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                    }
                    pub async fn main(
                        token: &String,
                        input: MainInput,
                    ) -> Result<MainOutput, XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.moderation.createReport"
                                .to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<MainOutput>().await
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
                    #[doc = "Rude, harassing, explicit, or otherwise unwelcoming behavior"]
                    pub struct ReasonRude;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    #[doc = "Other: reports not falling under another report category"]
                    pub struct ReasonOther;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    #[doc = "Spam: frequent unwanted promotion, replies, mentions"]
                    pub struct ReasonSpam;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    #[doc = "Unwanted or mis-labeled sexual content"]
                    pub struct ReasonSexual;
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
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    #[doc = "Direct violation of server rules, laws, terms of service"]
                    pub struct ReasonViolation;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    #[doc = "Misleading identity, affiliation, or content"]
                    pub struct ReasonMisleading;
                }
            }
            pub mod repo {
                #[allow(unused_imports)]
                use super::lexicon;
                pub mod delete_record {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub rkey: String,
                        pub collection: lexicon::nsid::Nsid,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub swap_record: Option<lexicon::cid::Cid>,
                        pub repo: lexicon::at_identifier::AtIdentifier,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub swap_commit: Option<lexicon::cid::Cid>,
                    }
                    pub async fn main(token: &String, input: MainInput) -> Result<(), XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.repo.deleteRecord".to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<()>().await
                    }
                }
                pub mod describe_repo {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub handle: lexicon::handle::Handle,
                        pub didDoc: ::serde_json::Value,
                        pub did: lexicon::did::Did,
                        pub collections: Vec<String>,
                        pub handle_is_correct: bool,
                    }
                    #[doc = "Description: \"Get information about the repo, including the list of collections.\""]
                    #[doc = "# Arguments"]
                    #[doc = "* `repo` - \"The handle or DID of the repo.\""]
                    pub async fn main(
                        token: &String,
                        repo: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.repo.describeRepo".to_string(),
                        );
                        req.param("repo".to_string(), repo.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod get_record {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub uri: lexicon::at_uri::AtUri,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cid: Option<lexicon::cid::Cid>,
                        pub value: ::serde_json::Value,
                    }
                    #[doc = "Description: \"Get a record.\""]
                    #[doc = "# Arguments"]
                    #[doc = "* `repo` - \"The handle or DID of the repo.\""]
                    #[doc = "* `rkey` - \"The key of the record.\""]
                    #[doc = "* `collection` - \"The NSID of the record collection.\""]
                    #[doc = "* `cid` - \"The CID of the version of the record. If not specified, then return the most recent version.\""]
                    pub async fn main(
                        token: &String,
                        cid: String,
                        collection: String,
                        repo: String,
                        rkey: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.repo.getRecord".to_string(),
                        );
                        req.param("collection".to_string(), collection.to_string());
                        req.param("cid".to_string(), cid.to_string());
                        req.param("repo".to_string(), repo.to_string());
                        req.param("rkey".to_string(), rkey.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
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
                        pub uri: lexicon::at_uri::AtUri,
                        pub cid: lexicon::cid::Cid,
                        pub value: ::serde_json::Value,
                    }
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub records: Vec<Record>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    #[doc = "Description: \"List a range of records in a collection.\""]
                    #[doc = "# Arguments"]
                    #[doc = "* `rkeyStart` - \"DEPRECATED: The lowest sort-ordered rkey to start from (exclusive)\""]
                    #[doc = "* `rkeyEnd` - \"DEPRECATED: The highest sort-ordered rkey to stop at (exclusive)\""]
                    #[doc = "* `reverse` - \"Reverse the order of the returned records?\""]
                    #[doc = "* `repo` - \"The handle or DID of the repo.\""]
                    #[doc = "* `limit` - \"The number of records to return.\""]
                    #[doc = "* `collection` - \"The NSID of the record type.\""]
                    pub async fn main(
                        token: &String,
                        collection: String,
                        cursor: String,
                        limit: i64,
                        repo: String,
                        reverse: bool,
                        rkey_end: String,
                        rkey_start: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.repo.listRecords".to_string(),
                        );
                        req.param("rkeyStart".to_string(), rkey_start.to_string());
                        req.param("cursor".to_string(), cursor.to_string());
                        req.param("collection".to_string(), collection.to_string());
                        req.param("rkeyEnd".to_string(), rkey_end.to_string());
                        req.param("reverse".to_string(), reverse.to_string());
                        req.param("repo".to_string(), repo.to_string());
                        req.param("limit".to_string(), limit.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
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
                        pub uri: lexicon::at_uri::AtUri,
                        pub cid: lexicon::cid::Cid,
                    }
                }
                pub mod upload_blob {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub blob: String,
                    }
                    pub async fn main(token: &String, input: ()) -> Result<MainOutput, XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.repo.uploadBlob".to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<MainOutput>().await
                    }
                }
                pub mod rebase_repo {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub repo: lexicon::at_identifier::AtIdentifier,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub swap_commit: Option<lexicon::cid::Cid>,
                    }
                    pub async fn main(token: &String, input: MainInput) -> Result<(), XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.repo.rebaseRepo".to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<()>().await
                    }
                }
                pub mod apply_writes {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
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
                        pub repo: lexicon::at_identifier::AtIdentifier,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub validate: Option<bool>,
                        pub writes: Vec<WritesType>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub swap_commit: Option<lexicon::cid::Cid>,
                    }
                    pub async fn main(token: &String, input: MainInput) -> Result<(), XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.repo.applyWrites".to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<()>().await
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Update {
                        pub value: ::serde_json::Value,
                        pub collection: lexicon::nsid::Nsid,
                        pub rkey: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Create {
                        pub collection: lexicon::nsid::Nsid,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub rkey: Option<String>,
                        pub value: ::serde_json::Value,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Delete {
                        pub rkey: String,
                        pub collection: lexicon::nsid::Nsid,
                    }
                }
                pub mod create_record {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub cid: lexicon::cid::Cid,
                        pub uri: lexicon::at_uri::AtUri,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub collection: lexicon::nsid::Nsid,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub swap_commit: Option<lexicon::cid::Cid>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub validate: Option<bool>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub rkey: Option<String>,
                        pub record: ::serde_json::Value,
                        pub repo: lexicon::at_identifier::AtIdentifier,
                    }
                    pub async fn main(
                        token: &String,
                        input: MainInput,
                    ) -> Result<MainOutput, XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.repo.createRecord".to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<MainOutput>().await
                    }
                }
                pub mod put_record {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub uri: lexicon::at_uri::AtUri,
                        pub cid: lexicon::cid::Cid,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub repo: lexicon::at_identifier::AtIdentifier,
                        pub record: ::serde_json::Value,
                        pub collection: lexicon::nsid::Nsid,
                        pub rkey: String,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub swap_record: Option<lexicon::cid::Cid>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub validate: Option<bool>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub swap_commit: Option<lexicon::cid::Cid>,
                    }
                    pub async fn main(
                        token: &String,
                        input: MainInput,
                    ) -> Result<MainOutput, XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.repo.putRecord".to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<MainOutput>().await
                    }
                }
            }
            pub mod server {
                #[allow(unused_imports)]
                use super::lexicon;
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
                    pub struct AppPassword {
                        pub password: String,
                        pub created_at: chrono::DateTime<chrono::Utc>,
                        pub name: String,
                    }
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub name: String,
                    }
                    pub async fn main(
                        token: &String,
                        input: MainInput,
                    ) -> Result<AppPassword, XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.server.createAppPassword"
                                .to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<AppPassword>().await
                    }
                }
                pub mod create_invite_code {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
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
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub for_account: Option<lexicon::did::Did>,
                    }
                    pub async fn main(
                        token: &String,
                        input: MainInput,
                    ) -> Result<MainOutput, XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.server.createInviteCode"
                                .to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<MainOutput>().await
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
                    pub struct AccountCodes {
                        pub codes: Vec<String>,
                        pub account: String,
                    }
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
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
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub for_accounts: Option<Vec<String>>,
                        pub code_count: i64,
                        pub use_count: i64,
                    }
                    pub async fn main(
                        token: &String,
                        input: MainInput,
                    ) -> Result<MainOutput, XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.server.createInviteCodes"
                                .to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<MainOutput>().await
                    }
                }
                pub mod create_session {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub did: lexicon::did::Did,
                        pub access_jwt: String,
                        pub refresh_jwt: String,
                        pub handle: lexicon::handle::Handle,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub email: Option<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub password: String,
                        pub identifier: String,
                    }
                    pub async fn main(
                        token: &String,
                        input: MainInput,
                    ) -> Result<MainOutput, XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.server.createSession".to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<MainOutput>().await
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
                    pub struct InviteCodeUse {
                        pub used_by: lexicon::did::Did,
                        pub used_at: chrono::DateTime<chrono::Utc>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct InviteCode {
                        pub available: i64,
                        pub created_by: String,
                        pub created_at: chrono::DateTime<chrono::Utc>,
                        pub code: String,
                        pub uses: Vec<InviteCodeUse>,
                        pub disabled: bool,
                        pub for_account: String,
                    }
                }
                pub mod delete_account {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub password: String,
                        pub token: String,
                        pub did: lexicon::did::Did,
                    }
                    pub async fn main(token: &String, input: MainInput) -> Result<(), XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.server.deleteAccount".to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<()>().await
                    }
                }
                pub mod delete_session {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    pub async fn main(token: &String, input: ()) -> Result<(), XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.server.deleteSession".to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<()>().await
                    }
                }
                pub mod describe_server {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub invite_code_required: Option<bool>,
                        pub available_user_domains: Vec<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub links: Option<Links>,
                    }
                    pub async fn main(token: &String) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.server.describeServer"
                                .to_string(),
                        );
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Links {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub privacy_policy: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub terms_of_service: Option<String>,
                    }
                }
                pub mod get_account_invite_codes {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub codes: Vec<lexicon::com::atproto::server::defs::InviteCode>,
                    }
                    pub async fn main(
                        token: &String,
                        create_available: bool,
                        include_used: bool,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.server.getAccountInviteCodes"
                                .to_string(),
                        );
                        req.param("createAvailable".to_string(), create_available.to_string());
                        req.param("includeUsed".to_string(), include_used.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod get_session {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub email: Option<String>,
                        pub did: lexicon::did::Did,
                        pub handle: lexicon::handle::Handle,
                    }
                    pub async fn main(token: &String) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.server.getSession".to_string(),
                        );
                        req.token(token);
                        req.execute::<MainOutput>().await
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
                        pub created_at: chrono::DateTime<chrono::Utc>,
                        pub name: String,
                    }
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub passwords: Vec<AppPassword>,
                    }
                    pub async fn main(token: &String) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.server.listAppPasswords"
                                .to_string(),
                        );
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod refresh_session {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub refresh_jwt: String,
                        pub handle: lexicon::handle::Handle,
                        pub access_jwt: String,
                        pub did: lexicon::did::Did,
                    }
                    pub async fn main(token: &String, input: ()) -> Result<MainOutput, XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.server.refreshSession"
                                .to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<MainOutput>().await
                    }
                }
                pub mod request_account_delete {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    pub async fn main(token: &String, input: ()) -> Result<(), XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.server.requestAccountDelete"
                                .to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<()>().await
                    }
                }
                pub mod request_password_reset {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub email: String,
                    }
                    pub async fn main(token: &String, input: MainInput) -> Result<(), XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.server.requestPasswordReset"
                                .to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<()>().await
                    }
                }
                pub mod reset_password {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub token: String,
                        pub password: String,
                    }
                    pub async fn main(token: &String, input: MainInput) -> Result<(), XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.server.resetPassword".to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<()>().await
                    }
                }
                pub mod revoke_app_password {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        pub name: String,
                    }
                    pub async fn main(token: &String, input: MainInput) -> Result<(), XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.server.revokeAppPassword"
                                .to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<()>().await
                    }
                }
                pub mod create_account {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub refresh_jwt: String,
                        pub access_jwt: String,
                        pub did: lexicon::did::Did,
                        pub handle: lexicon::handle::Handle,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub did: Option<lexicon::did::Did>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub invite_code: Option<String>,
                        pub password: String,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub recovery_key: Option<String>,
                        pub email: String,
                        pub handle: lexicon::handle::Handle,
                    }
                    pub async fn main(
                        token: &String,
                        input: MainInput,
                    ) -> Result<MainOutput, XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.server.createAccount".to_string(),
                        )
                        .input(input)
                        .token(token);
                        proc.execute::<MainOutput>().await
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
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[doc = "Description: \"Get a blob associated with a given repo.\""]
                    #[doc = "# Arguments"]
                    #[doc = "* `did` - \"The DID of the repo.\""]
                    #[doc = "* `cid` - \"The CID of the blob to fetch\""]
                    pub async fn main(
                        token: &String,
                        cid: String,
                        did: String,
                    ) -> Result<(), XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.getBlob".to_string(),
                        );
                        req.param("did".to_string(), did.to_string());
                        req.param("cid".to_string(), cid.to_string());
                        req.token(token);
                        req.execute::<()>().await
                    }
                }
                pub mod get_blocks {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[doc = "Description: \"Gets blocks from a given repo.\""]
                    #[doc = "# Arguments"]
                    #[doc = "* `did` - \"The DID of the repo.\""]
                    pub async fn main(
                        token: &String,
                        cids: Vec<String>,
                        did: String,
                    ) -> Result<(), XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.getBlocks".to_string(),
                        );
                        req.param("did".to_string(), did.to_string());
                        for ident in cids {
                            req.param("cids".to_string(), ident.to_string());
                        }
                        req.token(token);
                        req.execute::<()>().await
                    }
                }
                pub mod get_checkout {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[doc = "Description: \"Gets the repo state.\""]
                    #[doc = "# Arguments"]
                    #[doc = "* `did` - \"The DID of the repo.\""]
                    #[doc = "* `commit` - \"The commit to get the checkout from. Defaults to current HEAD.\""]
                    pub async fn main(
                        token: &String,
                        commit: String,
                        did: String,
                    ) -> Result<(), XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.getCheckout".to_string(),
                        );
                        req.param("commit".to_string(), commit.to_string());
                        req.param("did".to_string(), did.to_string());
                        req.token(token);
                        req.execute::<()>().await
                    }
                }
                pub mod get_commit_path {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub commits: Vec<String>,
                    }
                    #[doc = "Description: \"Gets the path of repo commits\""]
                    #[doc = "# Arguments"]
                    #[doc = "* `latest` - \"The most recent commit\""]
                    #[doc = "* `earliest` - \"The earliest commit to start from\""]
                    #[doc = "* `did` - \"The DID of the repo.\""]
                    pub async fn main(
                        token: &String,
                        did: String,
                        earliest: String,
                        latest: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.getCommitPath".to_string(),
                        );
                        req.param("did".to_string(), did.to_string());
                        req.param("latest".to_string(), latest.to_string());
                        req.param("earliest".to_string(), earliest.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod get_head {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub root: lexicon::cid::Cid,
                    }
                    #[doc = "Description: \"Gets the current HEAD CID of a repo.\""]
                    #[doc = "# Arguments"]
                    #[doc = "* `did` - \"The DID of the repo.\""]
                    pub async fn main(
                        token: &String,
                        did: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.getHead".to_string(),
                        );
                        req.param("did".to_string(), did.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod get_record {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[doc = "Description: \"Gets blocks needed for existence or non-existence of record.\""]
                    #[doc = "# Arguments"]
                    #[doc = "* `commit` - \"An optional past commit CID.\""]
                    #[doc = "* `did` - \"The DID of the repo.\""]
                    pub async fn main(
                        token: &String,
                        collection: String,
                        commit: String,
                        did: String,
                        rkey: String,
                    ) -> Result<(), XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.getRecord".to_string(),
                        );
                        req.param("collection".to_string(), collection.to_string());
                        req.param("commit".to_string(), commit.to_string());
                        req.param("did".to_string(), did.to_string());
                        req.param("rkey".to_string(), rkey.to_string());
                        req.token(token);
                        req.execute::<()>().await
                    }
                }
                pub mod get_repo {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[doc = "Description: \"Gets the repo state.\""]
                    #[doc = "# Arguments"]
                    #[doc = "* `did` - \"The DID of the repo.\""]
                    #[doc = "* `latest` - \"The latest commit in the commit range (inclusive)\""]
                    #[doc = "* `earliest` - \"The earliest commit in the commit range (not inclusive)\""]
                    pub async fn main(
                        token: &String,
                        did: String,
                        earliest: String,
                        latest: String,
                    ) -> Result<(), XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.getRepo".to_string(),
                        );
                        req.param("latest".to_string(), latest.to_string());
                        req.param("earliest".to_string(), earliest.to_string());
                        req.param("did".to_string(), did.to_string());
                        req.token(token);
                        req.execute::<()>().await
                    }
                }
                pub mod list_blobs {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        pub cids: Vec<String>,
                    }
                    #[doc = "Description: \"List blob cids for some range of commits\""]
                    #[doc = "# Arguments"]
                    #[doc = "* `latest` - \"The most recent commit\""]
                    #[doc = "* `earliest` - \"The earliest commit to start from\""]
                    #[doc = "* `did` - \"The DID of the repo.\""]
                    pub async fn main(
                        token: &String,
                        did: String,
                        earliest: String,
                        latest: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.listBlobs".to_string(),
                        );
                        req.param("latest".to_string(), latest.to_string());
                        req.param("earliest".to_string(), earliest.to_string());
                        req.param("did".to_string(), did.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                }
                pub mod list_repos {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                        pub repos: Vec<Repo>,
                    }
                    pub async fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.listRepos".to_string(),
                        );
                        req.param("limit".to_string(), limit.to_string());
                        req.param("cursor".to_string(), cursor.to_string());
                        req.token(token);
                        req.execute::<MainOutput>().await
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Repo {
                        pub did: lexicon::did::Did,
                        pub head: lexicon::cid::Cid,
                    }
                }
                pub mod notify_of_update {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[doc = "Description: \"Notify a crawling service of a recent update. Often when a long break between updates causes the connection with the crawling service to break.\""]
                    #[doc = "# Arguments"]
                    #[doc = "* `hostname` - \"Hostname of the service that is notifying of update.\""]
                    pub async fn main(token: &String, hostname: String) -> Result<(), XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.notifyOfUpdate".to_string(),
                        );
                        req.param("hostname".to_string(), hostname.to_string());
                        req.token(token);
                        req.execute::<()>().await
                    }
                }
                pub mod request_crawl {
                    #[allow(unused_imports)]
                    use super::lexicon;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    #[doc = "Description: \"Request a service to persistently crawl hosted repos.\""]
                    #[doc = "# Arguments"]
                    #[doc = "* `hostname` - \"Hostname of the service that is requesting to be crawled.\""]
                    pub async fn main(token: &String, hostname: String) -> Result<(), XrpcError> {
                        let mut req = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.requestCrawl".to_string(),
                        );
                        req.param("hostname".to_string(), hostname.to_string());
                        req.token(token);
                        req.execute::<()>().await
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
                        pub name: Name,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub message: Option<String>,
                    }
                    #[doc = "Subscribe to repo updates"]
                    use xrpc::error::XrpcError;
                    use xrpc::subscription::XrpcSubscription;
                    pub fn main(token: &String, cursor: i64) {
                        let mut req = XrpcSubscription::new(
                            "wss://bsky.social/xrpc/com.atproto.sync.subscribeRepos".to_string(),
                        );
                        req.param("cursor".to_string(), cursor.to_string());
                        req.token(token);
                        req.subscribe();
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Tombstone {
                        pub time: chrono::DateTime<chrono::Utc>,
                        pub seq: i64,
                        pub did: lexicon::did::Did,
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
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cid: Option<::serde_json::Value>,
                        pub action: Action,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Handle {
                        pub seq: i64,
                        pub did: lexicon::did::Did,
                        pub handle: lexicon::handle::Handle,
                        pub time: chrono::DateTime<chrono::Utc>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Commit {
                        pub blobs: Vec<Unimplemented>,
                        pub time: chrono::DateTime<chrono::Utc>,
                        pub too_big: bool,
                        pub repo: lexicon::did::Did,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub prev: Option<::serde_json::Value>,
                        pub blocks: ::serde_json::Value,
                        pub rebase: bool,
                        pub commit: ::serde_json::Value,
                        pub ops: Vec<RepoOp>,
                        pub seq: i64,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Migrate {
                        pub did: lexicon::did::Did,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub migrate_to: Option<String>,
                        pub seq: i64,
                        pub time: chrono::DateTime<chrono::Utc>,
                    }
                }
            }
        }
    }
}

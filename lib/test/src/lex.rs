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
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub avatar: Option<String>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        #[doc = "required: true"]
                        #[doc = "format: Handle"]
                        #[doc = "nullable: false"]
                        pub handle: String,
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub did: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub viewer: Option<ViewerState>,
                        #[doc = "max_graphemes: 64"]
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[doc = "max_length: 640"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub display_name: Option<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ProfileView {
                        #[doc = "max_graphemes: 64"]
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[doc = "max_length: 640"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub display_name: Option<String>,
                        #[doc = "max_graphemes: 256"]
                        #[doc = "max_length: 2560"]
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub description: Option<String>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Datetime"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub indexed_at: Option<String>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub avatar: Option<String>,
                        #[doc = "format: Handle"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub handle: String,
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
                        pub did: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub viewer: Option<ViewerState>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ViewerState {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub muted: Option<bool>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[doc = "format: AtUri"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub blocking: Option<String>,
                        #[doc = "format: AtUri"]
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub followed_by: Option<String>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub blocked_by: Option<bool>,
                        #[doc = "format: AtUri"]
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub following: Option<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ProfileViewDetailed {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub follows_count: Option<i64>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub posts_count: Option<i64>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub banner: Option<String>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub avatar: Option<String>,
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub indexed_at: Option<String>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub viewer: Option<ViewerState>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub followers_count: Option<i64>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub did: String,
                        #[doc = "max_length: 2560"]
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[doc = "max_graphemes: 256"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub description: Option<String>,
                        #[doc = "required: true"]
                        #[doc = "format: Handle"]
                        #[doc = "nullable: false"]
                        pub handle: String,
                        #[doc = "nullable: false"]
                        #[doc = "max_length: 640"]
                        #[doc = "required: false"]
                        #[doc = "max_graphemes: 64"]
                        #[serde(skip_serializing_if = "Option::is_none")]
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
                    pub async fn main(
                        token: &String,
                        actor: String,
                    ) -> Result<lexicon::app::bsky::actor::defs::ProfileViewDetailed, XrpcError>
                    {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.actor.getProfile".to_string(),
                        )
                        .param("actor".to_string(), actor.to_string())
                        .token(token);
                        query
                            .execute::<lexicon::app::bsky::actor::defs::ProfileViewDetailed>()
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
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub profiles: Vec<lexicon::app::bsky::actor::defs::ProfileViewDetailed>,
                    }
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        actors: Vec<String>,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.actor.getProfiles".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>().await
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub actors: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    #[doc = "Description: \"Get a list of actors suggested for following. Used in discovery UIs.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
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
                        query.execute::<MainOutput>().await
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
                        #[doc = "max_graphemes: 256"]
                        #[doc = "nullable: false"]
                        #[doc = "max_length: 2560"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub description: Option<String>,
                        #[doc = "accept: [\"image/png\", \"image/jpeg\"]"]
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub banner: Option<String>,
                        #[doc = "required: false"]
                        #[doc = "accept: [\"image/png\", \"image/jpeg\"]"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub avatar: Option<String>,
                        #[doc = "max_graphemes: 64"]
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[doc = "max_length: 640"]
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
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub actors: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    #[doc = "Description: \"Find actors matching search criteria.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
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
                        query.execute::<MainOutput>().await
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub actors: Vec<lexicon::app::bsky::actor::defs::ProfileViewBasic>,
                    }
                    #[doc = "Description: \"Find actor suggestions for a search term.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        limit: i64,
                        term: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.actor.searchActorsTypeahead"
                                .to_string(),
                        )
                        .param("term".to_string(), term.to_string())
                        .param("limit".to_string(), limit.to_string())
                        .token(token);
                        query.execute::<MainOutput>().await
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
                    pub struct ViewExternal {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub thumb: Option<String>,
                        #[doc = "format: Uri"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub uri: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub title: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub description: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct View {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub external: ViewExternal,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct External {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Uri"]
                        pub uri: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub title: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[doc = "accept: [\"image/*\"]"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub thumb: Option<String>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub description: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Main {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
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
                    pub struct Main {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub images: Vec<Image>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct View {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub images: Vec<ViewImage>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ViewImage {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub thumb: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub alt: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub fullsize: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Image {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub alt: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "accept: [\"image/*\"]"]
                        pub image: String,
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
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub embeds: Option<Vec<EmbedsType>>,
                        #[doc = "format: AtUri"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub uri: String,
                        #[doc = "nullable: false"]
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        pub indexed_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub value: ::serde_json::Value,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub author: lexicon::app::bsky::actor::defs::ProfileViewBasic,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Cid"]
                        pub cid: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Main {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub record: lexicon::com::atproto::repo::strong_ref::Main,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ViewNotFound {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: AtUri"]
                        pub uri: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ViewBlocked {
                        #[doc = "required: true"]
                        #[doc = "format: AtUri"]
                        #[doc = "nullable: false"]
                        pub uri: String,
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub media: MainmediaType,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub record: lexicon::app::bsky::embed::record::View,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
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
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Cid"]
                        pub cid: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub viewer: Option<ViewerState>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub author: lexicon::app::bsky::actor::defs::ProfileViewBasic,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub reply_count: Option<i64>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub like_count: Option<i64>,
                        #[doc = "nullable: false"]
                        #[doc = "format: AtUri"]
                        #[doc = "required: true"]
                        pub uri: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub embed: Option<PostViewembedType>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub repost_count: Option<i64>,
                        #[doc = "required: true"]
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        pub indexed_at: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub record: ::serde_json::Value,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ViewerState {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[doc = "format: AtUri"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub like: Option<String>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[doc = "format: AtUri"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub repost: Option<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct BlockedPost {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "const: true"]
                        pub blocked: bool,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: AtUri"]
                        pub uri: String,
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub post: lexicon::app::bsky::feed::defs::PostView,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub reason: Option<FeedViewPostreasonType>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub reply: Option<ReplyRef>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ReasonRepost {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub by: lexicon::app::bsky::actor::defs::ProfileViewBasic,
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub indexed_at: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct NotFoundPost {
                        #[doc = "nullable: false"]
                        #[doc = "format: AtUri"]
                        #[doc = "required: true"]
                        pub uri: String,
                        #[doc = "const: true"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub not_found: bool,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ReplyRef {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub parent: lexicon::app::bsky::feed::defs::PostView,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub root: lexicon::app::bsky::feed::defs::PostView,
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
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub replies: Option<Vec<RepliesType>>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub parent: Option<ThreadViewPostparentType>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub post: PostView,
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub feed: Vec<lexicon::app::bsky::feed::defs::FeedViewPost>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    #[doc = "Description: \"A view of an actor's feed.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
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
                        query.execute::<MainOutput>().await
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub likes: Vec<Like>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                        #[doc = "format: Cid"]
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cid: Option<String>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: AtUri"]
                        pub uri: String,
                    }
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
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
                        query.execute::<MainOutput>().await
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
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
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub thread: MainOutputthreadType,
                    }
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        depth: i64,
                        uri: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.feed.getPostThread".to_string(),
                        )
                        .param("uri".to_string(), uri.to_string())
                        .param("depth".to_string(), depth.to_string())
                        .token(token);
                        query.execute::<MainOutput>().await
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub posts: Vec<lexicon::app::bsky::feed::defs::PostView>,
                    }
                    #[doc = "Description: \"A view of an actor's feed.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        uris: Vec<String>,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.feed.getPosts".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>().await
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub reposted_by: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[doc = "format: Cid"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cid: Option<String>,
                        #[doc = "format: AtUri"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub uri: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        cid: String,
                        cursor: String,
                        limit: i64,
                        uri: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.feed.getRepostedBy".to_string(),
                        )
                        .param("uri".to_string(), uri.to_string())
                        .param("cursor".to_string(), cursor.to_string())
                        .param("cid".to_string(), cid.to_string())
                        .param("limit".to_string(), limit.to_string())
                        .token(token);
                        query.execute::<MainOutput>().await
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub feed: Vec<lexicon::app::bsky::feed::defs::FeedViewPost>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    #[doc = "Description: \"A view of the user's home timeline.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        algorithm: String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.feed.getTimeline".to_string(),
                        )
                        .param("limit".to_string(), limit.to_string())
                        .param("algorithm".to_string(), algorithm.to_string())
                        .param("cursor".to_string(), cursor.to_string())
                        .token(token);
                        query.execute::<MainOutput>().await
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub subject: lexicon::com::atproto::repo::strong_ref::Main,
                        #[doc = "nullable: false"]
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        pub created_at: String,
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
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub embed: Option<MainembedType>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub facets: Option<Vec<lexicon::app::bsky::richtext::facet::Main>>,
                        #[doc = "description: \"Deprecated: replaced by app.bsky.richtext.facet.\""]
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub entities: Option<Vec<Entity>>,
                        #[doc = "max_length: 3000"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "max_graphemes: 300"]
                        pub text: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub reply: Option<ReplyRef>,
                        #[doc = "required: true"]
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        pub created_at: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Entity {
                        #[doc = "description: \"Expected values are 'mention' and 'link'.\""]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub r#type: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub value: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub index: TextSlice,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ReplyRef {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub root: lexicon::com::atproto::repo::strong_ref::Main,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub parent: lexicon::com::atproto::repo::strong_ref::Main,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct TextSlice {
                        #[doc = "minimum: 0"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub start: i64,
                        #[doc = "minimum: 0"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
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
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Main {
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub created_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub subject: lexicon::com::atproto::repo::strong_ref::Main,
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
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Datetime"]
                        pub created_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Did"]
                        pub subject: String,
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
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub subject: String,
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub created_at: String,
                    }
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub blocks: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    #[doc = "Description: \"Who is the requester's account blocking?\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.graph.getBlocks".to_string(),
                        )
                        .param("limit".to_string(), limit.to_string())
                        .param("cursor".to_string(), cursor.to_string())
                        .token(token);
                        query.execute::<MainOutput>().await
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub followers: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub subject: lexicon::app::bsky::actor::defs::ProfileView,
                    }
                    #[doc = "Description: \"Who is following an actor?\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
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
                        query.execute::<MainOutput>().await
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
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub subject: lexicon::app::bsky::actor::defs::ProfileView,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub follows: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                    }
                    #[doc = "Description: \"Who is an actor following?\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
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
                        query.execute::<MainOutput>().await
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub mutes: Vec<lexicon::app::bsky::actor::defs::ProfileView>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    #[doc = "Description: \"Who does the viewer mute?\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.graph.getMutes".to_string(),
                        )
                        .param("limit".to_string(), limit.to_string())
                        .param("cursor".to_string(), cursor.to_string())
                        .token(token);
                        query.execute::<MainOutput>().await
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: AtIdentifier"]
                        pub actor: String,
                    }
                    #[doc = "Description: \"Mute an actor by did or handle.\""]
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
                        #[doc = "required: true"]
                        #[doc = "format: AtIdentifier"]
                        #[doc = "nullable: false"]
                        pub actor: String,
                    }
                    #[doc = "Description: \"Unmute an actor by did or handle.\""]
                    pub async fn main(token: &String, input: MainInput) -> Result<(), XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/app.bsky.graph.unmuteActor".to_string(),
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
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub count: i64,
                    }
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        seen_at: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.notification.getUnreadCount"
                                .to_string(),
                        )
                        .param("seenAt".to_string(), seen_at.to_string())
                        .token(token);
                        query.execute::<MainOutput>().await
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub notifications: Vec<Notification>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                        seen_at: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.notification.listNotifications"
                                .to_string(),
                        )
                        .param("limit".to_string(), limit.to_string())
                        .param("cursor".to_string(), cursor.to_string())
                        .param("seenAt".to_string(), seen_at.to_string())
                        .token(token);
                        query.execute::<MainOutput>().await
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub author: lexicon::app::bsky::actor::defs::ProfileView,
                        #[doc = "required: true"]
                        #[doc = "format: Cid"]
                        #[doc = "nullable: false"]
                        pub cid: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        #[doc = "nullable: false"]
                        #[doc = "description: \"Expected values are 'like', 'repost', 'follow', 'mention', 'reply', and 'quote'.\""]
                        #[doc = "required: true"]
                        #[doc = "known_values: [\"like\", \"repost\", \"follow\", \"mention\", \"reply\", \"quote\"]"]
                        pub reason: Reason,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub record: ::serde_json::Value,
                        #[doc = "format: AtUri"]
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub reason_subject: Option<String>,
                        #[doc = "required: true"]
                        #[doc = "format: AtUri"]
                        #[doc = "nullable: false"]
                        pub uri: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub is_read: bool,
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub indexed_at: String,
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Datetime"]
                        pub seen_at: String,
                    }
                    #[doc = "Description: \"Notify server that the user has seen notifications.\""]
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub index: ByteSlice,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub features: Vec<FeaturesType>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Link {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Uri"]
                        pub uri: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ByteSlice {
                        #[doc = "minimum: 0"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub byte_end: i64,
                        #[doc = "required: true"]
                        #[doc = "minimum: 0"]
                        #[doc = "nullable: false"]
                        pub byte_start: i64,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Mention {
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
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
                        Debug, Clone, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    pub struct Unimplemented;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub feed: Vec<lexicon::app::bsky::feed::defs::FeedViewPost>,
                    }
                    #[doc = "Description: \"An unspecced view of globally popular items\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        cursor: String,
                        include_nsfw: bool,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/app.bsky.unspecced.getPopular".to_string(),
                        )
                        .param("limit".to_string(), limit.to_string())
                        .param("includeNsfw".to_string(), include_nsfw.to_string())
                        .param("cursor".to_string(), cursor.to_string())
                        .token(token);
                        query.execute::<MainOutput>().await
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
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub codes: Option<Vec<String>>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub accounts: Option<Vec<String>>,
                    }
                    #[doc = "Description: \"Disable some set of codes and/or all codes associated with a set of users\""]
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
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub codes: Vec<lexicon::com::atproto::server::defs::InviteCode>,
                    }
                    #[doc = "Description: \"Admin view of invite codes\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                        sort: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.admin.getInviteCodes".to_string(),
                        )
                        .param("sort".to_string(), sort.to_string())
                        .param("cursor".to_string(), cursor.to_string())
                        .param("limit".to_string(), limit.to_string())
                        .token(token);
                        query.execute::<MainOutput>().await
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
                    pub async fn main(
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
                        query
                            .execute::<lexicon::com::atproto::admin::defs::ActionViewDetail>()
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
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub actions: Vec<lexicon::com::atproto::admin::defs::ActionView>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    #[doc = "Description: \"List moderation actions related to a subject.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                        subject: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.admin.getModerationActions"
                                .to_string(),
                        )
                        .param("cursor".to_string(), cursor.to_string())
                        .param("subject".to_string(), subject.to_string())
                        .param("limit".to_string(), limit.to_string())
                        .token(token);
                        query.execute::<MainOutput>().await
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
                    pub async fn main(
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
                        query
                            .execute::<lexicon::com::atproto::admin::defs::ReportViewDetail>()
                            .await
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
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub reports: Vec<lexicon::com::atproto::admin::defs::ReportView>,
                    }
                    #[doc = "Description: \"List moderation reports related to a subject.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
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
                        .param("cursor".to_string(), cursor.to_string())
                        .param("subject".to_string(), subject.to_string())
                        .param("limit".to_string(), limit.to_string())
                        .token(token);
                        query.execute::<MainOutput>().await
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
                    pub async fn main(
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
                        query
                            .execute::<lexicon::com::atproto::admin::defs::RecordViewDetail>()
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
                    #[doc = "Description: \"View details about a repository.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        did: String,
                    ) -> Result<lexicon::com::atproto::admin::defs::RepoViewDetail, XrpcError>
                    {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.admin.getRepo".to_string(),
                        )
                        .param("did".to_string(), did.to_string())
                        .token(token);
                        query
                            .execute::<lexicon::com::atproto::admin::defs::RepoViewDetail>()
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
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
                        pub created_by: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub action_id: i64,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub report_ids: Vec<i64>,
                    }
                    #[doc = "Description: \"Resolve moderation reports by an action.\""]
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub id: i64,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub reason: String,
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
                        pub created_by: String,
                    }
                    #[doc = "Description: \"Reverse a moderation action.\""]
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
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub repos: Vec<lexicon::com::atproto::admin::defs::RepoView>,
                    }
                    #[doc = "Description: \"Find repositories based on a search term.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
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
                        .param("cursor".to_string(), cursor.to_string())
                        .param("limit".to_string(), limit.to_string())
                        .param("invitedBy".to_string(), invited_by.to_string())
                        .token(token);
                        query.execute::<MainOutput>().await
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub reason: String,
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub created_by: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub subject: MainInputsubjectType,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub subject_blob_cids: Option<Vec<String>>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub create_label_vals: Option<Vec<String>>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "known_values: [\"com.atproto.admin.defs#takedown\", \"com.atproto.admin.defs#flag\", \"com.atproto.admin.defs#acknowledge\"]"]
                        pub action: Action,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub negate_label_vals: Option<Vec<String>>,
                    }
                    #[doc = "Description: \"Take a moderation action on a repo.\""]
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
                        #[doc = "description: \"The handle or DID of the repo.\""]
                        #[doc = "format: AtIdentifier"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub account: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub email: String,
                    }
                    #[doc = "Description: \"Administrative action to update an account's email\""]
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
                        #[doc = "format: Handle"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub handle: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Did"]
                        pub did: String,
                    }
                    #[doc = "Description: \"Administrative action to update an account's handle\""]
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
                    pub struct VideoDetails {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub width: i64,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub height: i64,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub length: i64,
                    }
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    #[doc = "Moderation action type: Acknowledge. Indicates that the content was reviewed and not considered to violate PDS rules."]
                    pub struct Acknowledge;
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub size: i64,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub details: Option<BlobViewdetailsType>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub moderation: Option<Moderation>,
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub created_at: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub mime_type: String,
                        #[doc = "format: Cid"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub cid: String,
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub action: ActionType,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub negate_label_vals: Option<Vec<String>>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub reason: String,
                        #[doc = "nullable: false"]
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        pub created_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub id: i64,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub create_label_vals: Option<Vec<String>>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub subject_blob_cids: Vec<String>,
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub created_by: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub subject: ActionViewsubjectType,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub reversal: Option<ActionReversal>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub resolved_report_ids: Vec<i64>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RepoViewDetail {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub invites: Option<Vec<lexicon::com::atproto::server::defs::InviteCode>>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub related_records: Vec<Unimplemented>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Did"]
                        pub did: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub email: Option<String>,
                        #[doc = "required: true"]
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        pub indexed_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub invited_by: Option<lexicon::com::atproto::server::defs::InviteCode>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub moderation: ModerationDetail,
                        #[doc = "format: Handle"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub handle: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RecordView {
                        #[doc = "nullable: false"]
                        #[doc = "format: AtUri"]
                        #[doc = "required: true"]
                        pub uri: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub blob_cids: Vec<String>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Datetime"]
                        pub indexed_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub moderation: Moderation,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub value: ::serde_json::Value,
                        #[doc = "required: true"]
                        #[doc = "format: Cid"]
                        #[doc = "nullable: false"]
                        pub cid: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub repo: RepoView,
                    }
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    #[doc = "Moderation action type: Flag. Indicates that the content was reviewed and considered to violate PDS rules, but may still be served."]
                    pub struct Flag;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Moderation {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub current_action: Option<ActionViewCurrent>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RepoView {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub related_records: Vec<Unimplemented>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Did"]
                        pub did: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub invited_by: Option<lexicon::com::atproto::server::defs::InviteCode>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Datetime"]
                        pub indexed_at: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub moderation: Moderation,
                        #[doc = "format: Handle"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub handle: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub email: Option<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RepoRef {
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub did: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ActionViewCurrent {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub id: i64,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub action: ActionType,
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
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct RecordViewDetail {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub moderation: ModerationDetail,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub labels: Option<Vec<lexicon::com::atproto::label::defs::Label>>,
                        #[doc = "required: true"]
                        #[doc = "format: AtUri"]
                        #[doc = "nullable: false"]
                        pub uri: String,
                        #[doc = "required: true"]
                        #[doc = "format: Cid"]
                        #[doc = "nullable: false"]
                        pub cid: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub value: ::serde_json::Value,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub blobs: Vec<BlobView>,
                        #[doc = "nullable: false"]
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        pub indexed_at: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub repo: RepoView,
                    }
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    #[doc = "Moderation action type: Escalate. Indicates that the content has been flagged for additional review."]
                    pub struct Escalate;
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
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Datetime"]
                        pub created_at: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub resolved_by_action_ids: Vec<i64>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub reason: Option<String>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub id: i64,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub subject: ReportViewsubjectType,
                        #[doc = "required: true"]
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        pub reported_by: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ActionReversal {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub reason: String,
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub created_at: String,
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub created_by: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ModerationDetail {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub actions: Vec<ActionView>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub current_action: Option<ActionViewCurrent>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub reports: Vec<ReportView>,
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
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub reversal: Option<ActionReversal>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub negate_label_vals: Option<Vec<String>>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub resolved_reports: Vec<ReportView>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub subject: ActionViewDetailsubjectType,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub reason: String,
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub created_by: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub action: ActionType,
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub created_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub id: i64,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub create_label_vals: Option<Vec<String>>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub subject_blobs: Vec<BlobView>,
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
                        #[serde(alias = "com.atproto.admin.defs#recordView")]
                        RecordView(Box<RecordView>),
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ReportViewDetail {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub resolved_by_actions:
                            Vec<lexicon::com::atproto::admin::defs::ActionView>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub reason: Option<String>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Datetime"]
                        pub created_at: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub subject: ReportViewDetailsubjectType,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub id: i64,
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub reported_by: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct ImageDetails {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub height: i64,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub width: i64,
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
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub did: String,
                    }
                    #[doc = "Description: \"Provides the DID of a repo.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        handle: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.identity.resolveHandle"
                                .to_string(),
                        )
                        .param("handle".to_string(), handle.to_string())
                        .token(token);
                        query.execute::<MainOutput>().await
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
                        #[doc = "format: Handle"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub handle: String,
                    }
                    #[doc = "Description: \"Updates the handle of the account\""]
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
                        #[doc = "required: true"]
                        #[doc = "max_length: 128"]
                        #[doc = "description: \"the short string name of the value or type of this label\""]
                        #[doc = "nullable: false"]
                        pub val: String,
                        #[doc = "format: Uri"]
                        #[doc = "description: \"AT URI of the record, repository (account), or other resource which this label applies to\""]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub uri: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[doc = "description: \"optionally, CID specifying the specific version of 'uri' resource this label applies to\""]
                        #[doc = "format: Cid"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cid: Option<String>,
                        #[doc = "description: \"if true, this is a negation label, overwriting a previous label\""]
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub neg: Option<bool>,
                        #[doc = "required: true"]
                        #[doc = "description: \"timestamp when this label was created\""]
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        pub cts: String,
                        #[doc = "description: \"DID of the actor who created this label\""]
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    #[doc = "Description: \"Find labels relevant to the provided URI patterns.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                        sources: Vec<String>,
                        uri_patterns: Vec<String>,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.label.queryLabels".to_string(),
                        )
                        .param("limit".to_string(), limit.to_string())
                        .param("cursor".to_string(), cursor.to_string())
                        .token(token);
                        query.execute::<MainOutput>().await
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub labels: Vec<lexicon::com::atproto::label::defs::Label>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub seq: i64,
                    }
                    #[doc = "Subscribe to label updates"]
                    use xrpc::error::XrpcError;
                    use xrpc::subscription::XrpcSubscription;
                    pub fn main(token: &String, cursor: i64) {
                        let query = XrpcSubscription::new(
                            "https://bsky.social/xrpc/com.atproto.label.subscribeLabels"
                                .to_string(),
                        )
                        .param("cursor".to_string(), cursor.to_string())
                        .token(token);
                        query.subscribe();
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
                        #[doc = "known_values: [\"OutdatedCursor\"]"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub name: Name,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
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
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub reason: Option<String>,
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub reported_by: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Datetime"]
                        pub created_at: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub subject: MainOutputsubjectType,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub id: i64,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
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
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub reason: Option<String>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub subject: MainInputsubjectType,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub reason_type: lexicon::com::atproto::moderation::defs::ReasonType,
                    }
                    #[doc = "Description: \"Report a repo or a record.\""]
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
                    #[doc = "Spam: frequent unwanted promotion, replies, mentions"]
                    pub struct ReasonSpam;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    #[doc = "Other: reports not falling under another report category"]
                    pub struct ReasonOther;
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
                    #[doc = "Misleading identity, affiliation, or content"]
                    pub struct ReasonMisleading;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    #[doc = "Unwanted or mis-labeled sexual content"]
                    pub struct ReasonSexual;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    #[doc = "Rude, harassing, explicit, or otherwise unwelcoming behavior"]
                    pub struct ReasonRude;
                    #[derive(
                        Debug, Clone, PartialEq, Eq, :: serde :: Deserialize, :: serde :: Serialize,
                    )]
                    #[doc = "Direct violation of server rules, laws, terms of service"]
                    pub struct ReasonViolation;
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
                        #[doc = "nullable: false"]
                        #[doc = "description: \"Validate the records?\""]
                        #[doc = "required: false"]
                        #[doc = "default: true"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub validate: Option<bool>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub writes: Vec<WritesType>,
                        #[doc = "description: \"The handle or DID of the repo.\""]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: AtIdentifier"]
                        pub repo: String,
                        #[doc = "nullable: false"]
                        #[doc = "format: Cid"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub swap_commit: Option<String>,
                    }
                    #[doc = "Description: \"Apply a batch transaction of creates, updates, and deletes.\""]
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
                    pub struct Create {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Nsid"]
                        pub collection: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub value: ::serde_json::Value,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub rkey: Option<String>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Delete {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Nsid"]
                        pub collection: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub rkey: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Update {
                        #[doc = "format: Nsid"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub collection: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub rkey: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub value: ::serde_json::Value,
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
                        #[doc = "format: AtUri"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub uri: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Cid"]
                        pub cid: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        #[doc = "format: AtIdentifier"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "description: \"The handle or DID of the repo.\""]
                        pub repo: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[doc = "description: \"The key of the record.\""]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub rkey: Option<String>,
                        #[doc = "default: true"]
                        #[doc = "description: \"Validate the record?\""]
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub validate: Option<bool>,
                        #[doc = "required: true"]
                        #[doc = "format: Nsid"]
                        #[doc = "description: \"The NSID of the record collection.\""]
                        #[doc = "nullable: false"]
                        pub collection: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub record: ::serde_json::Value,
                        #[doc = "description: \"Compare and swap with the previous commit by cid.\""]
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[doc = "format: Cid"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub swap_commit: Option<String>,
                    }
                    #[doc = "Description: \"Create a new record.\""]
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
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Nsid"]
                        #[doc = "description: \"The NSID of the record collection.\""]
                        pub collection: String,
                        #[doc = "format: Cid"]
                        #[doc = "description: \"Compare and swap with the previous record by cid.\""]
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub swap_record: Option<String>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "description: \"The handle or DID of the repo.\""]
                        #[doc = "format: AtIdentifier"]
                        pub repo: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "description: \"The key of the record.\""]
                        pub rkey: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[doc = "description: \"Compare and swap with the previous commit by cid.\""]
                        #[doc = "format: Cid"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub swap_commit: Option<String>,
                    }
                    #[doc = "Description: \"Delete a record, or ensure it doesn't exist.\""]
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
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub didDoc: ::serde_json::Value,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        pub did: String,
                        #[doc = "format: Handle"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub handle: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub collections: Vec<String>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub handle_is_correct: bool,
                    }
                    #[doc = "Description: \"Get information about the repo, including the list of collections.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        repo: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.repo.describeRepo".to_string(),
                        )
                        .param("repo".to_string(), repo.to_string())
                        .token(token);
                        query.execute::<MainOutput>().await
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
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub value: ::serde_json::Value,
                        #[doc = "required: false"]
                        #[doc = "format: Cid"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cid: Option<String>,
                        #[doc = "nullable: false"]
                        #[doc = "format: AtUri"]
                        #[doc = "required: true"]
                        pub uri: String,
                    }
                    #[doc = "Description: \"Get a record.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        cid: String,
                        collection: String,
                        repo: String,
                        rkey: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.repo.getRecord".to_string(),
                        )
                        .param("collection".to_string(), collection.to_string())
                        .param("repo".to_string(), repo.to_string())
                        .param("rkey".to_string(), rkey.to_string())
                        .param("cid".to_string(), cid.to_string())
                        .token(token);
                        query.execute::<MainOutput>().await
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
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub records: Vec<Record>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    #[doc = "Description: \"List a range of records in a collection.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
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
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.repo.listRecords".to_string(),
                        )
                        .param("limit".to_string(), limit.to_string())
                        .param("cursor".to_string(), cursor.to_string())
                        .param("repo".to_string(), repo.to_string())
                        .param("rkeyStart".to_string(), rkey_start.to_string())
                        .param("reverse".to_string(), reverse.to_string())
                        .param("collection".to_string(), collection.to_string())
                        .param("rkeyEnd".to_string(), rkey_end.to_string())
                        .token(token);
                        query.execute::<MainOutput>().await
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Record {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub value: ::serde_json::Value,
                        #[doc = "format: Cid"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub cid: String,
                        #[doc = "format: AtUri"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
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
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "format: AtUri"]
                        #[doc = "required: true"]
                        pub uri: String,
                        #[doc = "format: Cid"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub cid: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        #[doc = "description: \"The NSID of the record collection.\""]
                        #[doc = "required: true"]
                        #[doc = "format: Nsid"]
                        #[doc = "nullable: false"]
                        pub collection: String,
                        #[doc = "format: AtIdentifier"]
                        #[doc = "description: \"The handle or DID of the repo.\""]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub repo: String,
                        #[doc = "required: true"]
                        #[doc = "description: \"The key of the record.\""]
                        #[doc = "nullable: false"]
                        pub rkey: String,
                        #[doc = "description: \"Validate the record?\""]
                        #[doc = "default: true"]
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub validate: Option<bool>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub record: ::serde_json::Value,
                        #[doc = "description: \"Compare and swap with the previous record by cid.\""]
                        #[doc = "nullable: true"]
                        #[doc = "format: Cid"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub swap_record: Option<String>,
                        #[doc = "format: Cid"]
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[doc = "description: \"Compare and swap with the previous commit by cid.\""]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub swap_commit: Option<String>,
                    }
                    #[doc = "Description: \"Write a record, creating or updating it as needed.\""]
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: AtUri"]
                        pub uri: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Cid"]
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
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub blob: String,
                    }
                    #[doc = "Description: \"Upload a new blob to be added to repo in a later request.\""]
                    pub async fn main(token: &String, input: ()) -> Result<MainOutput, XrpcError> {
                        let proc = XrpcProcedure::new(
                            "https://bsky.social/xrpc/com.atproto.repo.uploadBlob".to_string(),
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
                        #[doc = "required: true"]
                        #[doc = "format: Handle"]
                        #[doc = "nullable: false"]
                        pub handle: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub access_jwt: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub refresh_jwt: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        pub did: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Handle"]
                        pub handle: String,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub invite_code: Option<String>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub recovery_key: Option<String>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub email: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub password: String,
                    }
                    #[doc = "Description: \"Create an account.\""]
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
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub password: String,
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub created_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub name: String,
                    }
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub name: String,
                    }
                    #[doc = "Description: \"Create an app-specific password.\""]
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub code: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub use_count: i64,
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub for_account: Option<String>,
                    }
                    #[doc = "Description: \"Create an invite code.\""]
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
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub codes: Vec<AccountCodes>,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub for_accounts: Option<Vec<String>>,
                        #[doc = "default: 1"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub code_count: i64,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub use_count: i64,
                    }
                    #[doc = "Description: \"Create an invite code.\""]
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
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct AccountCodes {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub account: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
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
                    use xrpc::error::XrpcError;
                    use xrpc::procedure::XrpcProcedure;
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub access_jwt: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub email: Option<String>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub refresh_jwt: String,
                        #[doc = "format: Handle"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub handle: String,
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub did: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainInput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub password: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "description: \"Handle or other identifier supported by the server for the authenticating user.\""]
                        pub identifier: String,
                    }
                    #[doc = "Description: \"Create an authentication session.\""]
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
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub used_by: String,
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub used_at: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct InviteCode {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub created_by: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub available: i64,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub code: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub disabled: bool,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub for_account: String,
                        #[doc = "format: Datetime"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub created_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub uses: Vec<InviteCodeUse>,
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
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub password: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub token: String,
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub did: String,
                    }
                    #[doc = "Description: \"Delete a user account with a token and password.\""]
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
                    #[doc = "Description: \"Delete the current session.\""]
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
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub links: Option<Links>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub available_user_domains: Vec<String>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub invite_code_required: Option<bool>,
                    }
                    #[doc = "Description: \"Get a document describing the service's accounts configuration.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(token: &String) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.server.describeServer"
                                .to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>().await
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Links {
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub terms_of_service: Option<String>,
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub privacy_policy: Option<String>,
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub codes: Vec<lexicon::com::atproto::server::defs::InviteCode>,
                    }
                    #[doc = "Description: \"Get all invite codes for a given account\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        create_available: bool,
                        include_used: bool,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.server.getAccountInviteCodes"
                                .to_string(),
                        )
                        .param("includeUsed".to_string(), include_used.to_string())
                        .param("createAvailable".to_string(), create_available.to_string())
                        .token(token);
                        query.execute::<MainOutput>().await
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
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub email: Option<String>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Did"]
                        pub did: String,
                        #[doc = "nullable: false"]
                        #[doc = "format: Handle"]
                        #[doc = "required: true"]
                        pub handle: String,
                    }
                    #[doc = "Description: \"Get information about the current session.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(token: &String) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.server.getSession".to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>().await
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
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub created_at: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub name: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct MainOutput {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub passwords: Vec<AppPassword>,
                    }
                    #[doc = "Description: \"List all app-specific passwords.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(token: &String) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.server.listAppPasswords"
                                .to_string(),
                        )
                        .token(token);
                        query.execute::<MainOutput>().await
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
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub did: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub refresh_jwt: String,
                        #[doc = "format: Handle"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub handle: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub access_jwt: String,
                    }
                    #[doc = "Description: \"Refresh an authentication session.\""]
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
                    #[doc = "Description: \"Initiate a user account deletion via email.\""]
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub email: String,
                    }
                    #[doc = "Description: \"Initiate a user account password reset via email.\""]
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub token: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub password: String,
                    }
                    #[doc = "Description: \"Reset a user account password using a token.\""]
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub name: String,
                    }
                    #[doc = "Description: \"Revoke an app-specific password by name.\""]
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
                    pub async fn main(
                        token: &String,
                        cid: String,
                        did: String,
                    ) -> Result<(), XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.getBlob".to_string(),
                        )
                        .param("did".to_string(), did.to_string())
                        .param("cid".to_string(), cid.to_string())
                        .token(token);
                        query.execute::<()>().await
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
                    pub async fn main(
                        token: &String,
                        cids: Vec<String>,
                        did: String,
                    ) -> Result<(), XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.getBlocks".to_string(),
                        )
                        .param("did".to_string(), did.to_string())
                        .token(token);
                        query.execute::<()>().await
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
                    pub async fn main(
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
                        query.execute::<()>().await
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
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub commits: Vec<String>,
                    }
                    #[doc = "Description: \"Gets the path of repo commits\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        did: String,
                        earliest: String,
                        latest: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.getCommitPath".to_string(),
                        )
                        .param("earliest".to_string(), earliest.to_string())
                        .param("did".to_string(), did.to_string())
                        .param("latest".to_string(), latest.to_string())
                        .token(token);
                        query.execute::<MainOutput>().await
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
                        #[doc = "nullable: false"]
                        #[doc = "format: Cid"]
                        #[doc = "required: true"]
                        pub root: String,
                    }
                    #[doc = "Description: \"Gets the current HEAD CID of a repo.\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        did: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.getHead".to_string(),
                        )
                        .param("did".to_string(), did.to_string())
                        .token(token);
                        query.execute::<MainOutput>().await
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
                    pub async fn main(
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
                        .param("did".to_string(), did.to_string())
                        .param("commit".to_string(), commit.to_string())
                        .param("collection".to_string(), collection.to_string())
                        .token(token);
                        query.execute::<()>().await
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
                    pub async fn main(
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
                        query.execute::<()>().await
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
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub cids: Vec<String>,
                    }
                    #[doc = "Description: \"List blob cids for some range of commits\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        did: String,
                        earliest: String,
                        latest: String,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.listBlobs".to_string(),
                        )
                        .param("earliest".to_string(), earliest.to_string())
                        .param("latest".to_string(), latest.to_string())
                        .param("did".to_string(), did.to_string())
                        .token(token);
                        query.execute::<MainOutput>().await
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
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub repos: Vec<Repo>,
                        #[doc = "nullable: false"]
                        #[doc = "required: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cursor: Option<String>,
                    }
                    #[doc = "Description: \"List dids and root cids of hosted repos\""]
                    use xrpc::error::XrpcError;
                    use xrpc::query::XrpcQuery;
                    pub async fn main(
                        token: &String,
                        cursor: String,
                        limit: i64,
                    ) -> Result<MainOutput, XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.listRepos".to_string(),
                        )
                        .param("cursor".to_string(), cursor.to_string())
                        .param("limit".to_string(), limit.to_string())
                        .token(token);
                        query.execute::<MainOutput>().await
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Repo {
                        #[doc = "nullable: false"]
                        #[doc = "format: Cid"]
                        #[doc = "required: true"]
                        pub head: String,
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        #[doc = "required: true"]
                        pub did: String,
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
                    pub async fn main(token: &String, hostname: String) -> Result<(), XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.notifyOfUpdate".to_string(),
                        )
                        .param("hostname".to_string(), hostname.to_string())
                        .token(token);
                        query.execute::<()>().await
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
                    pub async fn main(token: &String, hostname: String) -> Result<(), XrpcError> {
                        let query = XrpcQuery::new(
                            "https://bsky.social/xrpc/com.atproto.sync.requestCrawl".to_string(),
                        )
                        .param("hostname".to_string(), hostname.to_string())
                        .token(token);
                        query.execute::<()>().await
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
                    pub struct Handle {
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub time: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub seq: i64,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Handle"]
                        pub handle: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        pub did: String,
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
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub path: String,
                        #[doc = "nullable: true"]
                        #[doc = "required: true"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub cid: Option<::serde_json::Value>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "known_values: [\"create\", \"update\", \"delete\"]"]
                        pub action: Action,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Migrate {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub seq: i64,
                        #[doc = "required: true"]
                        #[doc = "format: Did"]
                        #[doc = "nullable: false"]
                        pub did: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: true"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub migrate_to: Option<String>,
                        #[doc = "required: true"]
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        pub time: String,
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Commit {
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub blobs: Vec<Unimplemented>,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub commit: ::serde_json::Value,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub too_big: bool,
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub time: String,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub seq: i64,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub rebase: bool,
                        #[doc = "nullable: true"]
                        #[doc = "required: true"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub prev: Option<::serde_json::Value>,
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        #[doc = "format: Did"]
                        pub repo: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub blocks: ::serde_json::Value,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub ops: Vec<RepoOp>,
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
                        #[doc = "required: false"]
                        #[doc = "nullable: false"]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub message: Option<String>,
                        #[doc = "known_values: [\"OutdatedCursor\"]"]
                        #[doc = "nullable: false"]
                        #[doc = "required: true"]
                        pub name: Name,
                    }
                    #[doc = "Subscribe to repo updates"]
                    use xrpc::error::XrpcError;
                    use xrpc::subscription::XrpcSubscription;
                    pub async fn main(token: &String, cursor: i64) {
                        let query = XrpcSubscription::new(
                            "https://bsky.social/xrpc/com.atproto.sync.subscribeRepos".to_string(),
                        )
                        .param("cursor".to_string(), cursor.to_string())
                        .token(token);
                        query.subscribe().await;
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, :: serde :: Serialize, :: serde :: Deserialize,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct Tombstone {
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        #[doc = "format: Did"]
                        pub did: String,
                        #[doc = "required: true"]
                        #[doc = "format: Datetime"]
                        #[doc = "nullable: false"]
                        pub time: String,
                        #[doc = "required: true"]
                        #[doc = "nullable: false"]
                        pub seq: i64,
                    }
                }
            }
        }
    }
}

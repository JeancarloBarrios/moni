use crate::data_store::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, default, string};

use crate::client::Client;
use tokio::time::{sleep, Duration};
const BASE_SCOPE: &str = "https://www.googleapis.com/auth/cloud-platform";

pub struct DataStoreClient {
    client: Client,
}

impl DataStoreClient {
    pub async fn new() -> Result<Self, Error> {
        let client = Client::new().await.map_err(Error::ClientError)?;
        Ok(Self { client })
    }

    /// # Create Data Store
    /// Creates a `DataStore` for storing documents, with the option to configure it for advanced site search.
    /// This function constructs and sends a POST request to the Discovery Engine's DataStore creation endpoint.
    ///
    /// # Parameters
    /// - `request`: A `CreateDataStoreRequest` containing:
    ///   - `data_store`: The data store details to be created.
    ///   - `project_id`: The project identifier.
    ///   - `collections`: The collection associated with the data store.
    ///   - `data_store_id`: The identifier for the data store, conforming to RFC-1034 with a 63 character limit.
    ///   - `create_advance_site_search`: Optional boolean flag indicating whether to create an advanced data store for site search.
    ///
    /// # Returns
    /// Returns an `Operation` if successful or a `VertexError` in case of an error.
    ///
    /// # Examples
    /// ```
    /// ```
    ///
    /// Note: The endpoint URL is built using the project ID, location ("global" by default), and collection name.
    pub async fn create_data_store(
        &self,
        request: CreateDataStoreRequest,
    ) -> Result<Operation, Error> {
        let location = "global";
        let create_advance_site_search = request.create_advance_site_search.unwrap_or(false);

        let url = reqwest::Url::parse_with_params(
            format!(
                "https://discoveryengine.googleapis.com/v1beta/projects/{}/locations/{}/collections/{}/dataStores",
                request.project_id, location, request.collections
            )
            .as_str(),
            &[("dataStoreId", request.data_store_id), ("createAdvancedSiteSearch", create_advance_site_search.to_string())],
        );

        let response = self
            .client
            .api_post(&[BASE_SCOPE], url.unwrap().as_str(), request.data_store)
            .await
            .map_err(Error::ClientError)?
            .error_for_status()
            .map_err(Error::HttpStatus)?;

        let operation: Operation = response.json().await.map_err(Error::ResponseJsonParsing)?;

        Ok(operation)
    }

    // Sets up a Google cloud storage data store
    pub async fn setup_data_connector(
        &self,
        request: SetupDataConnectorRequest,
    ) -> Result<SetupDataConnectorResponse, Error> {
        let location = "global";

        let url = reqwest::Url::parse(
            format!(
                "https://discoveryengine.googleapis.com/v1/projects/{}/locations/{}/global:setUpDataConnector",
                request.project_id, location,
            )
                .as_str(),
        );

        let response = self
            .client
            .api_post(&[BASE_SCOPE], url.unwrap().as_str(), request)
            .await
            .map_err(Error::ClientError)?
            .error_for_status()
            .map_err(Error::HttpStatus)?;

        let operation: SetupDataConnectorResponse =
            response.json().await.map_err(Error::ResponseJsonParsing)?;

        Ok(operation)
    }

    /// # Delete Data Store
    /// Deletes a `DataStore`.
    ///
    /// This function constructs and sends a DELETE request to the Discovery Engine's DataStore deletion endpoint.
    ///
    /// # Parameters
    /// - `request`: A `DeleteDataStoreRequest` containing:
    ///   - `project_id`: The project identifier.
    ///   - `collections`: The collection associated with the data store.
    ///   - `data_store_id`: The identifier for the data store.
    ///
    /// # Returns
    /// Returns an `Operation` if successful or an `Error` in case of an error.
    ///
    /// # HTTP Request
    /// DELETE `https://discoveryengine.googleapis.com/v1/projects/{project}/locations/{location}/collections/{collection}/dataStores`
    ///
    /// The URL uses gRPC Transcoding syntax. The location is set to "global" by default.
    ///
    /// # Authorization Scopes
    /// Requires the following OAuth scope:
    /// - `https://www.googleapis.com/auth/cloud-platform`
    ///
    /// For more information, see the [Authentication Overview](https://cloud.google.com/docs/authentication).
    ///
    /// # IAM Permissions
    /// Requires the following IAM permission on the `name` resource:
    /// - `discoveryengine.dataStores.delete`
    ///
    /// For more information, see the [IAM documentation](https://cloud.google.com/iam/docs/).
    ///
    /// # Examples
    ///
    /// Note: Ensure that the `request` parameter is correctly formatted with the project ID, collection, and data store ID.
    pub async fn delete_data_store(
        &self,
        request: DeleteDataStoreRequest,
    ) -> Result<Operation, Error> {
        let location = "global";
        let url = format!(
                "https://discoveryengine.googleapis.com/v1/projects/{}/locations/{}/collections/{}/dataStores/{}",
                request.project_id, location, request.collections, request.data_store_id
            );
        let response = self
            .client
            .api_delete(&[BASE_SCOPE], &url, None)
            .await
            .map_err(Error::ClientError)?
            .error_for_status()
            .map_err(Error::HttpStatus)?;
        let operation: Operation = response.json().await.map_err(Error::ResponseJsonParsing)?;
        Ok(operation)
    }

    /// # Get Data Store
    /// Retrieves a `DataStore`.
    /// This function constructs and sends a GET request to the Discovery Engine's DataStore retrieval endpoint.
    ///
    /// # Parameters
    /// - `request`: A `GetDataStoreRequest` containing:
    ///  - `project_id`: The project identifier.
    ///  - `collections`: The collection associated with the data store.
    ///  - `data_store_id`: The identifier for the data store.
    ///
    ///  # Returns
    ///  Returns a `DataStore` if successful or an `Error` in case of an error.
    //
    ///  # HTTP Request
    ///  GET `https://discoveryengine.googleapis.com/v1/projects/{project}/locations/{location}/collections/{collection}/dataStores`
    /// The URL uses gRPC Transcoding syntax. The location is set to "global" by default.
    ///
    /// # Authorization Scopes
    /// Requires the following OAuth scope:
    /// - `https://www.googleapis.com/auth/cloud-platform`
    /// For more information, see the [Authentication Overview](https://cloud.google.com/docs/authentication).
    ///
    /// # IAM Permissions
    /// Requires the following IAM permission on the `name` resource:
    /// - `discoveryengine.dataStores.get`
    /// For more information, see the [IAM documentation](https://cloud.google.com/iam/docs/).
    ///
    /// # Examples
    ///    Note: Ensure that the `request` parameter is correctly formatted with the project ID, collection, and data store ID.
    pub async fn get_data_store(&self, request: GetDataStoreRequest) -> Result<DataStore, Error> {
        let location = "global";
        let url = format!(
                "https://discoveryengine.googleapis.com/v1/projects/{}/locations/{}/collections/{}/dataStores",
                request.project_id, location, request.collections
            );
        let response = self
            .client
            .api_get_with_params(
                &[BASE_SCOPE],
                &url,
                Some([("data_store_id", request.data_store_id.as_str())].to_vec()),
            )
            .await
            .map_err(Error::ClientError)?
            .error_for_status()
            .map_err(Error::HttpStatus)?;
        let data_store: DataStore = response.json().await.map_err(Error::ResponseJsonParsing)?;
        Ok(data_store)
    }

    /// # List Chunks
    /// Lists the chunks in a document.
    /// This function constructs and sends a GET request to the Discovery Engine's chunk listing endpoint.
    ///
    /// # Parameters
    /// - `request`: A `ListChunksRequest` containing:
    ///  - `project_id`: The project identifier.
    ///  - `collections`: The collection associated with the data store.
    ///  - `data_store_id`: The identifier for the data store.
    ///  - `branch`: The branch identifier.
    ///  - `documet_id`: The document identifier.
    ///
    ///  # Returns
    ///  Returns a `ListChunksResponse` if successful or an `Error` in case of an error.
    ///
    ///  # HTTP Request
    ///  GET `https://discoveryengine.googleapis.com/v1/projects/{project}/locations/{location}/collections/{collection}/dataStores/{dataStore}/branches/{branch}/documents/{document}/chunks`
    ///  The URL uses gRPC Transcoding syntax. The location is set to "global" by default.
    ///  # Authorization Scopes
    ///  Requires the following OAuth scope:
    ///  - `https://www.googleapis.com/auth/cloud-platform`
    ///  For more information, see the [Authentication Overview](https://cloud.google.com/docs/authentication).
    ///
    ///  # IAM Permissions
    ///  Requires the following IAM permission on the `name` resource:
    ///  - `discoveryengine.dataStores.chunks.list`
    ///  For more information, see the [IAM documentation](https://cloud.google.com/iam/docs/).
    ///
    ///  Note: Ensure that the `request` parameter is correctly formatted with the project ID, collection, data store ID, branch, and document ID.

    pub async fn search_chunks(
        &self,
        request: SearchChunksRequest,
    ) -> Result<SearchChunksResponse, Error> {
        let location = "global";

        let url = format!(
            "https://discoveryengine.googleapis.com/v1alpha/projects/{}/locations/{}/collections/{}/dataStores/{}/servingConfigs/default_search:search",
            request.project_id, location, request.collections, request.data_store_id
        );
        let response = self
            .client
            .api_get_with_params(&[BASE_SCOPE], &url, None)
            .await
            .map_err(Error::ClientError)?
            .error_for_status()
            .map_err(Error::HttpStatus)?;
        let search_chunks_response: SearchChunksResponse =
            response.json().await.map_err(Error::ResponseJsonParsing)?;
        Ok(search_chunks_response)
    }

    pub async fn search(&self, request: SearchRequest) -> Result<SearchResponse, Error> {
        let location = "global";
        let app_id = "moni-demo-final_1722720080773";
        // let data_store = "moni-demo_1722720098936";
        let server_config = format!("projects/{}/locations/{}/collections/default_collection/engines/{}/servingConfigs/default_serving_config", request.project_id, location, app_id);
        let url = format!(
            "https://discoveryengine.googleapis.com/v1beta/{}:search",
            server_config
        );
        let response = self
            .client
            .api_post(&[BASE_SCOPE], &url, request.discovery_engine_search_request)
            .await
            .map_err(Error::ClientError)?
            .error_for_status()
            .map_err(Error::HttpStatus)?;

        let search_response: SearchResponse =
            response.json().await.map_err(Error::ResponseJsonParsing)?;
        Ok(search_response)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ListDocumentsResponse {
    documents: Vec<Document>,
    next_page_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Document {
    name: String,
    id: String,
    content: Option<Content>,
    parent_document_id: Option<String>,
    derived_struct_data: Option<serde_json::Value>,
    acl_info: Option<AclInfo>,
    index_time: Option<String>,
    #[serde(flatten)]
    data: Option<DocumentData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Content {
    mime_type: String,
    #[serde(flatten)]
    content: Option<ContentData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum ContentData {
    RawBytes { raw_bytes: String },
    Uri { uri: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AclInfo {
    readers: Option<Vec<AccessRestriction>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AccessRestriction {
    principals: Option<Vec<Principal>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Principal {
    #[serde(flatten)]
    principal: Option<PrincipalType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum PrincipalType {
    UserId { user_id: String },
    GroupId { group_id: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum DocumentData {
    StructData { struct_data: serde_json::Value },
    JsonData { json_data: String },
}
pub struct SearchRequest {
    pub project_id: String,
    pub discovery_engine_search_request: DiscoveryEngineSearchRequest,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SearchResponse {
    results: Option<Vec<SearchResult>>,
    facets: Option<Vec<Facet>>,
    guided_search_result: Option<GuidedSearchResult>,
    total_size: Option<i32>,
    attribution_token: Option<String>,
    redirect_uri: Option<String>,
    next_page_token: Option<String>,
    corrected_query: Option<String>,
    summary: Option<Summary>,
    applied_controls: Option<Vec<String>>,
    geo_search_debug_info: Option<Vec<GeoSearchDebugInfo>>,
    query_expansion_info: Option<QueryExpansionInfo>,
    natural_language_query_understanding_info: Option<NaturalLanguageQueryUnderstandingInfo>,
    session_info: Option<SessionInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct NaturalLanguageQueryUnderstandingInfo {
    extracted_filters: Option<String>,
    rewritten_query: Option<String>,
    structured_extracted_filter: Option<StructuredExtractedFilter>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct StructuredExtractedFilter {
    expression: Option<Expression>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
enum Expression {
    StringConstraint { string_constraint: StringConstraint },
    NumberConstraint { number_constraint: NumberConstraint },
    GeolocationConstraint { geolocation_constraint: GeolocationConstraint },
    AndExpr { and_expr: AndExpression },
    OrExpr { or_expr: OrExpression },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct StringConstraint {
    field_name: String,
    values: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct NumberConstraint {
    field_name: String,
    comparison: Comparison,
    value: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum Comparison {
    COMPARISON_UNSPECIFIED,
    EQUALS,
    LESS_THAN_EQUALS,
    LESS_THAN,
    GREATER_THAN_EQUALS,
    GREATER_THAN,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GeolocationConstraint {
    field_name: String,
    address: String,
    radius_in_meters: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AndExpression {
    expressions: Vec<Expression>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct OrExpression {
    expressions: Vec<Expression>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct QueryExpansionInfo {
    expanded_query: bool,
    pinned_result_count: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GeoSearchDebugInfo {
    original_address_query: String,
    error_message: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Summary {
    summary_text: Option<String>,
    summary_skipped_reasons: Option<Vec<SummarySkippedReason>>,
    safety_attributes: Option<SafetyAttributes>,
    summary_with_metadata: Option<SummaryWithMetadata>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum SummarySkippedReason {
    #[default]
    SummarySkippedReasonUnspecified,
    AdversarialQueryIgnored,
    NonSummarySeekingQueryIgnored,
    OutOfDomainQueryIgnored,
    PotentialPolicyViolation,
    LlmAddonNotEnabled,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SafetyAttributes {
    categories: Option<Vec<String>>,
    scores: Option<Vec<f64>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SummaryWithMetadata {
    summary: String,
    citation_metadata: Option<CitationMetadata>,
    references: Option<Vec<Reference>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CitationMetadata {
    citations: Option<Vec<Citation>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Citation {
    start_index: String,
    end_index: String,
    sources: Option<Vec<CitationSource>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CitationSource {
    reference_index: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Reference {
    title: Option<String>,
    document: String,
    uri: Option<String>,
    chunk_contents: Option<Vec<ChunkContent>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ChunkContent {
    content: String,
    page_identifier: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GuidedSearchResult {
    refinement_attributes: Option<Vec<RefinementAttribute>>,
    follow_up_questions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RefinementAttribute {
    attribute_key: String,
    attribute_value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Facet {
    key: String,
    values: Vec<FacetValue>,
    dynamic_facet: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FacetValue {
    count: String,
    #[serde(flatten)]
    facet_value: FacetValueType,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum FacetValueType {
    Value { value: String },
    Interval { interval: Interval },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SearchResult {
    id: Option<String>,
    document: Option<Document>,
    chunk: Option<Chunk>,
    model_scores: Option<HashMap<String, DoubleList>>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DoubleList {
    values: Option<Vec<f64>>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Snippet {
    snippet_status: String,
    snippet: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ExtractiveAnswer {
    page_number: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SessionInfo {
    name: String,
    query_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveryEngineSearchRequest {
    pub branch: String,
    pub query: String,
    pub image_query: ImageQuery,
    pub page_size: u32,
    pub page_token: String,
    pub offset: u32,
    pub data_store_specs: Vec<DataStoreSpec>,
    pub filter: String,
    pub canonical_filter: String,
    pub order_by: String,
    pub user_info: UserInfo,
    pub language_code: String,
    pub facet_specs: Vec<FacetSpec>,
    pub boost_spec: BoostSpec,
    pub params: HashMap<String, Value>,
    pub query_expansion_spec: QueryExpansionSpec,
    pub spell_correction_spec: SpellCorrectionSpec,
    pub user_pseudo_id: String,
    pub content_search_spec: ContentSearchSpec,
    pub safe_search: bool,
    pub user_labels: HashMap<String, Value>,
    pub search_as_you_type_spec: SearchAsYouTypeSpec,
    pub session: String,
    pub session_spec: SessionSpec,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SessionSpec {
    pub query_id: String,
    pub search_result_persistence_count: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchAsYouTypeSpec {
    pub condition: SearchAsYouTypeCondition,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContentSearchSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet_spec: Option<SnippetSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_spec: Option<SummarySpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_spec: Option<ChunkSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extractive_content_spec: Option<ExtractiveContentSpec>,
    pub search_result_mode: SearchResultMode,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SearchResultMode {
    #[default]
    SearchResultModeUnspecified,
    Documents,
    Chunks,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SummarySpec {
    pub summary_result_count: u32,
    pub include_citations: bool,
    pub ignore_adversarial_query: bool,
    pub ignore_non_summary_seeking_query: bool,
    pub model_prompt_spec: ModelPromptSpec,
    pub language_mode: String,
    pub model_spec: ModelSpec,
    pub use_semantic_chunks: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ModelPromptSpec {
    pub preamble: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ModelSpec {
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExtractiveContentSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_extractive_answer_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_extractive_segment_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_extractive_segment_score: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_previus_segments: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_next_segments: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SnippetSpec {
    pub max_snippet_count: i32,
    pub reference_only: bool,
    pub return_snippet: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SpellCorrectionSpec {
    pub mode: Mode,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Mode {
    ModeUnspecified,
    SugestionOnly,
    #[default]
    Auto,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct BoostSpec {
    pub condition_boost_specs: Vec<ConditionBoostSpec>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConditionBoostSpec {
    pub condition: String,
    pub boost: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ControlPoint {
    pub attribute_value: String,
    pub boost_amount: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AttributeType {
    AttributeTypeUnspecified,
    Numerical,
    Freshness,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InterpolationType {
    InterpolationTypeUnspecified,
    Linear,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImageQuery {
    pub image_bytes: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataStoreSpec {
    pub data_store: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub user_id: String,
    pub user_agent: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FacetSpec {
    pub facet_key: FacetKey,
    pub limit: i32,
    pub excluded_filter_keys: Vec<String>,
    pub enable_dynamic_position: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FacetKey {
    pub key: String,
    pub interval: Vec<Interval>,
    pub restricted_values: Vec<String>,
    pub prefixes: Vec<String>,
    pub contains: Vec<String>,
    pub case_insensitve: bool,
    pub order_by: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Interval {
    pub minimum: i32,
    pub exclusive_minimum: i32,
    pub maximum: i32,
    pub exclusive_maximum: i32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryExpansionSpec {
    pub condition: Condition,
    pub pin_unexpanded_results: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SearchAsYouTypeCondition {
    ConditionUnspecified,
    #[default]
    Disabled,
    Enabled,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Condition {
    ConditionUnspecified,
    Disabled,
    #[default]
    Auto,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetupDataConnectorResponse {
    pub name: String,
    pub response: ResponseDataConnector,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseDataConnector {
    #[serde(rename = "@type")]
    pub type_url: String,
    pub name: String,
    pub state: String,
    pub data_source: String,
    pub params: Params,
    pub refresh_interval: String,
    pub entities: Vec<ResponseEntity>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseEntity {
    pub entity_name: String,
    pub data_store: String,
    pub params: EntityParams,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetupDataConnectorRequest {
    pub project_id: String,
    pub collection_id: String,
    pub collection_display_name: String,
    pub data_connector: DataConnector,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataConnector {
    pub data_source: String,
    pub params: Params,
    pub refresh_interval: String,
    pub entities: Vec<Entity>,
    pub sync_mode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    pub instance_uris: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    pub entity_name: String,
    pub params: EntityParams,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntityParams {
    pub data_schema: String,
    pub content_config: String,
    pub industry_vertical: String,
    pub auto_generate_ids: bool,
}

pub struct ListChunksRequest {
    pub project_id: String,
    pub collections: String,
    pub data_store_id: String,
    pub branch: String,
    pub documet_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListChunksResponse {
    pub chunks: Vec<Chunk>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChunkSpec {
    pub num_previous_chunks: Option<i32>,
    pub num_next_chunks: Option<i32>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchChunksRequest {
    pub project_id: String,
    pub collections: String,
    pub data_store_id: String,
    pub serving_config: String,
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    pub content_search_spec: ContentSearchSpec,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchChunksResponse {
    pub chunks: Vec<Chunk>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Chunk {
    pub name: String,
    pub id: String,
    pub content: String,
    #[serde(rename = "documentMetadata")]
    pub document_metadata: DocumentMetadata,
    #[serde(rename = "deriveStructData")]
    pub derive_struct_data: HashMap<String, Value>,
    #[serde(rename = "pageSpan")]
    pub page_span: PageSpan,
    #[serde(rename = "chunkMetadata")]
    pub chunk_metadata: ChunkMetadata,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "relevanceScore")]
    relevance_score: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentMetadata {
    pub uri: String,
    pub title: String,
    #[serde(rename = "structData")]
    pub struct_data: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageSpan {
    #[serde(rename = "pageStart")]
    pub page_start: i32,
    #[serde(rename = "pageEnd")]
    pub page_end: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChunkMetadata {
    #[serde(rename = "previusChunks")]
    pub previus_chunks: Vec<Chunk>,
    #[serde(rename = "nextChunks")]
    pub next_chunks: Vec<Chunk>,
}

pub struct GetDataStoreRequest {
    pub collections: String,
    pub project_id: String,
    pub data_store_id: String,
}

pub struct DeleteDataStoreRequest {
    pub collections: String,
    pub project_id: String,
    pub data_store_id: String,
}

pub struct CreateDataStoreRequest {
    pub data_store: DataStore,
    pub project_id: String,
    pub collections: String,
    pub data_store_id: String,
    pub create_advance_site_search: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetOperationStatusRequest {
    pub operation_name: String,
    pub project_id: String,
    pub collection: String,
    pub data_store_id: String,
    pub branch: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PollOperationRequest {
    pub operation_name: String,
    pub project_id: String,
    pub collection: String,
    pub data_store_id: String,
    pub branch: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Operation {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    pub done: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationError {
    pub code: i32,
    pub message: String,
    pub details: Vec<HashMap<String, serde_json::Value>>, // Adjust as needed
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "@type")]
    pub at_type: String,
    #[serde(flatten)]
    pub additional: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "@type")]
    pub at_type: String,
    #[serde(flatten)]
    pub additional: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub code: i32,
    pub message: String,
    pub details: Vec<Detail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Detail {
    #[serde(rename = "@type")]
    pub at_type: String,
    #[serde(flatten)]
    pub additional: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OperationResult {
    Error { error: Status },
    Response { response: Response },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataStore {
    pub name: String,
    pub display_name: String,
    pub industry_vertical: IndustryVertical,
    pub solution_types: Vec<SolutionType>,
    pub default_schema_id: Option<String>,
    pub content_config: ContentConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_info: Option<LanguageInfo>,
    pub document_processing_config: Option<DocumentProcessingConfig>,
    pub starting_schema: Option<Schema>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IndustryVertical {
    Unspecified,
    Media,
    SiteSearch,
    Generic,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SolutionType {
    Unspecified,
    Recommendation,
    Search,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContentConfig {
    Unspecified,
    NoContent,
    ContentRequired,
    PublicWebsite,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LanguageInfo {
    pub language_code: String,
    pub normalized_language_code: Option<String>,
    pub language: Option<String>,
    pub region: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentProcessingConfig {
    pub name: String,
    pub chunking_config: Option<ChunkingConfig>,
    pub default_parsing_config: Option<ParsingConfig>,
    pub parsing_config_overrides: Option<HashMap<String, ParsingConfig>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChunkingConfig {
    pub layout_based_chunking_config: Option<LayoutBasedChunkingConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LayoutBasedChunkingConfig {
    pub chunk_size: Option<i32>,
    pub include_ancestor_headings: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParsingConfig {
    pub digital_parsing_config: Option<DigitalParsingConfig>,
    pub ocr_parsing_config: Option<OcrParsingConfig>,
    pub layout_parsing_config: Option<LayoutParsingConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DigitalParsingConfig {}

#[derive(Serialize, Deserialize, Debug)]
pub struct OcrParsingConfig {
    pub enhanced_document_elements: Option<Vec<String>>,
    pub use_native_text: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LayoutParsingConfig {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Schema {}

// Test
#[cfg(test)]
mod tests_integrations {
    use crate::client;

    use super::*;
    use rand::{self, Rng};
    use std::{env, thread};

    // Test token_provider
    // #[tokio::test]
    // async fn test_token_provider() {
    //     env::set_var(
    //         "GOOGLE_APPLICATION_CREDENTIALS",
    //         "../../private/gcp_key.json",
    //     );
    //     // load file
    //     let token_provider = token_provider().await;
    //     assert!(token_provider.token(&[BASE_SCOPE]).await.is_ok());
    //     let token = token_provider.token(&[BASE_SCOPE]).await.unwrap();
    //     assert!(!token.as_str().is_empty());
    // }

    // Test create_data_store
    #[tokio::test]
    async fn test_create_data_store() {
        env::set_var(
            "GOOGLE_APPLICATION_CREDENTIALS",
            "../../private/gcp_key.json",
        );
        let mut rng = rand::thread_rng();
        let random_number: u32 = rng.gen_range(1000..10000); // Generates a random number between 1000 and 9999

        let random_name = format!("moni-test-{}", random_number);
        let project_id = "moni-429523";
        let collections = "default_collection";
        let data_store = DataStore {
            name: random_name.to_string(),
            display_name: random_name.to_string(),
            industry_vertical: IndustryVertical::Generic,
            solution_types: vec![],
            default_schema_id: None,
            content_config: ContentConfig::PublicWebsite,
            create_time: None,
            language_info: None,
            document_processing_config: None,
            starting_schema: None,
        };

        let data_store_id = format!("moni-test-{}", random_number);

        let data_store_request = CreateDataStoreRequest {
            data_store,
            project_id: project_id.to_string(),
            collections: collections.to_string(),
            data_store_id: data_store_id.to_string(),
            create_advance_site_search: None,
        };

        let client = DataStoreClient::new().await.unwrap();

        let operation = client.create_data_store(data_store_request).await;

        println!("{:?}", operation);

        assert!(operation.is_ok());

        // let operation_resolved = operation.unwrap();
        // let operation_request = PollOperationRequest {
        //     operation_name: operation_resolved.name.to_string(),
        //     project_id: project_id.to_string(),
        //     collection: collections.to_string(),
        //     data_store_id: data_store_id.to_string(),
        //     branch: "default_branch".to_string(),
        // };
        // let operation_finished = client.poll_operation(operation_request, None, None).await;
        // assert!(operation_finished);
        // Now lets delete it
        thread::sleep(Duration::from_secs(5));
        let delete_request = DeleteDataStoreRequest {
            project_id: project_id.to_string(),
            collections: collections.to_string(),
            data_store_id: data_store_id.to_string(),
        };
        let delete_operation = client.delete_data_store(delete_request).await;

        println!("{:?}", delete_operation);
        assert!(delete_operation.is_ok());
        println!("{:?}", delete_operation.unwrap());
    }

    #[tokio::test]
    async fn test_search_document() {
        env::set_var(
            "GOOGLE_APPLICATION_CREDENTIALS",
            "../../private/gcp_key.json",
        );
        let project_id = "875055333740";
        let _collections = "default_collection";
        let _data_store_id = "moni-demo_1722720098936";

        let request = SearchRequest {
            project_id: project_id.to_string(),
            discovery_engine_search_request: DiscoveryEngineSearchRequest {
                session: "projects/875055333740/locations/global/collections/default_collection/engines/moni-demo-final_1722720080773/sessions/-".to_string(),
                query: "Can you show all document that a relevant for Colombian Climate adaptation"
                    .to_string(),
                page_size: 10,
                filter: "".to_string(),
                query_expansion_spec: QueryExpansionSpec {
                    condition: Condition::Auto,
                    ..Default::default()
                },
                spell_correction_spec: SpellCorrectionSpec { mode: Mode::Auto },
                content_search_spec: ContentSearchSpec {
                    extractive_content_spec: Some(ExtractiveContentSpec {
                        max_extractive_segment_count: Some(1),
                        ..Default::default()
                    }),
                    snippet_spec: Some(SnippetSpec {
                        max_snippet_count: 1,
                        return_snippet: true,
                        ..Default::default()
                    }),
                    chunk_spec: None,
                    ..Default::default()
                },
                session_spec: SessionSpec {
                    search_result_persistence_count: 5,
                    ..Default::default()
                },
                ..Default::default()
            },
        };

        let client = DataStoreClient::new().await.unwrap();
        let response = client.search(request).await;
        println!("{:?}", response);
        assert!(response.is_ok());
        let search_response = response.unwrap();

    }

    // Test create_data_store with a storage bucket.
    // #[tokio::test]
    // async fn test_create_data_store_storage_bucket() {
    //     env::set_var(
    //         "GOOGLE_APPLICATION_CREDENTIALS",
    //         "../../private/gcp_key.json",
    //     );
    //
    //     let project_id = "moni-429523";
    //     let collections = "default_collection";
    //     let data_store_id = "test-1-gcp";
    //
    //     let data_store_request = SetupDataConnectorRequest {
    //         project_id: project_id.to_string(),
    //         collection_id: "moni-demo-1_1722304644136".to_string(),
    //         collection_display_name: "moni-demo-1-gcs".to_string(),
    //         data_connector: DataConnector {
    //             data_source: "gcs".to_string(),
    //             params: Params {
    //                 instance_uris: vec!["gs://moni-demo-1".to_string()],
    //             },
    //             refresh_interval: "86400s".to_string(),
    //             entities: vec![Entity {
    //                 entity_name: "gcs_store".to_string(),
    //                 params: EntityParams {
    //                     data_schema: "content-with-faq-csv".to_string(),
    //                     content_config: "content_required".to_string(),
    //                     industry_vertical: "industry_vertical_unspecified".to_string(),
    //                     auto_generate_ids: false,
    //                 },
    //             }],
    //             sync_mode: "PERIODIC".to_string(),
    //         },
    //
    //     };
    //
    //     let setup_connector_response = DataStoreClient::new().await.unwrap().setup_data_connector(data_store_request).await;
    //
    //     let client = DataStoreClient::new().await.unwrap();
    //
    //     // Now lets delete it
    //     let delete_request = DeleteDataStoreRequest {
    //         project_id: project_id.to_string(),
    //         collections: collections.to_string(),
    //         data_store_id: data_store_id.to_string(),
    //     };
    //     let delete_operation = client.delete_data_store(delete_request).await;
    //
    //     println!("{:?}", delete_operation);
    //     assert!(delete_operation.is_ok());
    //     println!("{:?}", delete_operation.unwrap());
    // }
    //
    // #[tokio::test]
    // async fn test_search_data_store() {
    //     env::set_var(
    //         "GOOGLE_APPLICATION_CREDENTIALS",
    //         "../../private/gcp_key.json",
    //     );
    //
    //     let project_id = "moni-429523";
    //     let collections = "default_collection";
    //     let data_store = DataStore {
    //         name: "moni-test".to_string(),
    //         display_name: "moni-test".to_string(),
    //         industry_vertical: IndustryVertical::Generic,
    //         solution_types: vec![],
    //         default_schema_id: None,
    //         content_config: ContentConfig::PublicWebsite,
    //         create_time: None,
    //         language_info: None,
    //         document_processing_config: None,
    //         starting_schema: None,
    //     };
    //
    //     let data_store_id = "test-1";
    //
    //     let data_store_request = CreateDataStoreRequest {
    //         data_store,
    //         project_id: project_id.to_string(),
    //         collections: collections.to_string(),
    //         data_store_id: data_store_id.to_string(),
    //         create_advance_site_search: None,
    //     };
    //
    //     let client = DataStoreClient::new().await.unwrap();
    //     // let operation = client.create_data_store(data_store_request).await;
    //
    //     /* println!("{:?}", operation);
    //
    //     assert!(operation.is_ok());
    //
    //     println!("{:?}", operation.unwrap()); */
    //
    //     // Now lets delete it
    //     let delete_request = DeleteDataStoreRequest {
    //         project_id: project_id.to_string(),
    //         collections: collections.to_string(),
    //         data_store_id: data_store_id.to_string(),
    //     };
    //     let delete_operation = client.delete_data_store(delete_request).await;
    //
    //     println!("{:?}", delete_operation);
    //     assert!(delete_operation.is_ok());
    //     println!("{:?}", delete_operation
}
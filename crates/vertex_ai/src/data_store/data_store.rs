use crate::data_store::error::Error;
use std::collections::HashMap;
use gcloud_sdk::google::api::ProjectProperties;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use rand::Rng;

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
    /// let request = CreateDataStoreRequest {
    ///     data_store: DataStore{...},
    ///     project_id: "project123",
    ///     collections: "collection456",
    ///     data_store_id: "dataStore789",
    ///     create_sadvance_site_search: Some(true),
    /// };
    /// let operation = client.create_data_store(request).await?;
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
                "https://discoveryengine.googleapis.com/v1/projects/{}/locations/{}/collections/{}/dataStores",
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

        let operation: SetupDataConnectorResponse = response.json().await.map_err(Error::ResponseJsonParsing)?;

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
    /// ```
    /// let request = DeleteDataStoreRequest {
    ///     project_id: "project123".to_string(),
    ///     collections: "collection456".to_string(),
    ///     data_store_id: "dataStore789".to_string(),
    /// };
    /// let operation = client.delete_data_store(request).await?;
    /// ```
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
            .api_delete(
                &[BASE_SCOPE],
                &url,
                None,
            )
            .await
            .map_err(Error::ClientError)?
            .error_for_status()
            .map_err(Error::HttpStatus)?;
        let response_text = response.text().await;
        println!("Response text: {}", response_text.unwrap().to_string());
        // let operation: Operation = response.json().await.map_err(Error::ResponseJsonParsing)?;
        Ok(Operation {
            name: "test".to_string(),
            metadata: None,
            done: true,
            response: HashMap::new(),
        })
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
    /// ```
    /// let request = GetDataStoreRequest {
    ///    project_id: "project123".to_string(),
    ///    collections: "collection456".to_string(),
    ///    data_store_id: "dataStore789".to_string(),
    ///    };
    ///    let data_store = client.get_data_store(request).await?;
    ///    ```
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
    ///  # Examples
    ///  ```
    ///  let request = ListChunksRequest {
    ///  project_id: "project123".to_string(),
    ///  collections: "collection456".to_string(),
    ///  data_store_id: "dataStore789".to_string(),
    ///  branch: "branch123".to_string(),
    ///  documet_id: "document123".to_string(),
    ///  };
    ///  let chunks = client.list_chunks(request).await?;
    ///  ```
    ///  Note: Ensure that the `request` parameter is correctly formatted with the project ID, collection, data store ID, branch, and document ID.
    pub async fn get_operation_status(
        &self,
        request: GetOperationStatusRequest,
    ) -> Result<Operation, Box<dyn std::error::Error>> {
        let location = "global";

        let url = format!(
            "https://discoveryengine.googleapis.com/v1alpha/projects/{}/locations/{}/collections/{}/dataStores/{}/branches/{}/operations/{}",
            request.project_id, location, request.collection, request.data_store_id, request.branch, request.operation_id
        );

        let response = self
            .client
            .api_get_with_params(&["https://www.googleapis.com/auth/cloud-platform"], &url, None)
            .await
            .map_err(|err| Box::new(Error::ClientError(err)) as Box<dyn std::error::Error>)?
            .error_for_status()
            .map_err(|err| Box::new(Error::HttpStatus(err)))?;

        if response.status().is_success() {
            let operation: Operation = response.json().await?;
            Ok(operation)
        } else {
            let error = response.text().await?;
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error)))
        }
    }

    pub async fn poll_operation(&self, operation_name: PollOperationRequest, max_retries: Option<u32>, wait_time: Option<u64>) -> bool {
        let max_retries = max_retries.unwrap_or(20);
        let wait_time = wait_time.unwrap_or(5);
        let mut retries = 0;

        loop {

            let request = GetOperationStatusRequest {
                name: operation_name.name.to_string(),
                project_id: operation_name.project_id.to_string(),
                collection: operation_name.collection.to_string(),
                data_store_id: operation_name.data_store_id.to_string(),
                branch: operation_name.branch.to_string(),
                operation_id: operation_name.operation_id.to_string(),
            };
            let operation_status = self.get_operation_status(request).await;
            println!("polling operation {:?}", operation_status);
            println!();
            match operation_status {
                Ok(status) if status.done => return true,
                Ok(_) => {
                    retries += 1;
                    if retries >= max_retries {
                        return false;
                    }
                    sleep(Duration::from_secs(wait_time)).await; // Adjust the duration as needed
                }
                Err(_) => return false,
            }
        }
    }
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
pub struct  SetupDataConnectorRequest {
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
pub struct ContentSearchSpec {
    pub chunk_spec: Option<ChunkSpec>,
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
    pub content_search_spec: ContentSearchSpec

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
    pub name: String,
    pub project_id: String,
    pub collection: String,
    pub data_store_id: String,
    pub branch: String,
    pub operation_id: String,

}
#[derive(Serialize, Deserialize, Debug)]
pub struct PollOperationRequest {
    pub name: String,
    pub project_id: String,
    pub collection: String,
    pub data_store_id: String,
    pub branch: String,
    pub operation_id: String,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Operation {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    pub done: bool,
    pub response: HashMap<String, String>,
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
    use std::env;

    use super::*;

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

        let data_store_id = format!("moni-test-{}", random_number);;

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

        let operation_resolved = operation.unwrap();
        let operation_request = PollOperationRequest{
            name: operation_resolved.name.to_string(),
            project_id: project_id.to_string(),
            collection: collections.to_string(),
            data_store_id: data_store_id.to_string(),
            branch: "default_branch".to_string(),
            operation_id: operation_resolved.name.to_string(),
        };
        let operation_finished = client.poll_operation(operation_request, None, None).await;
        assert_eq!(operation_finished, true);
        // Now lets delete it
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
    //     println!("{:?}", delete_operation.unwrap());
    // }
}

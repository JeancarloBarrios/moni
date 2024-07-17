pub mod data_store;
pub mod error;
use std::{collections::HashMap, sync::Arc};

use serde_json::Value;

use error::VertexError;
use gcp_auth::TokenProvider;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;

const BASE_SCOPE: &str = "https://www.googleapis.com/auth/cloud-platform";

static TOKEN_PROVIDER: OnceCell<Arc<dyn TokenProvider>> = OnceCell::const_new();

// token_provider expect a enviorment variable called GOOGLE_APPLICATION_CREDENTIALS to be set
async fn token_provider() -> &'static Arc<dyn TokenProvider> {
    TOKEN_PROVIDER
        .get_or_init(|| async {
            gcp_auth::provider()
                .await
                .expect("unable to initialize token provider")
        })
        .await
}

#[derive(Clone)]
pub struct VertexClient {
    client: Client,
}

impl VertexClient {
    pub async fn new() -> Result<Self, VertexError> {
        let client = Client::new();
        Ok(Self { client })
    }

    async fn auth_headers(
        &self,
        scopes: &[&str],
    ) -> Result<reqwest::header::HeaderMap, VertexError> {
        let token_provider = token_provider().await;
        let token = token_provider
            .token(scopes)
            .await
            .map_err(VertexError::ProviderError)?;
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", token.as_str()).parse().unwrap(),
        );
        Ok(headers)
    }

    pub async fn api_post<T>(
        &self,
        scopes: &[&str],
        url: &str,
        body: T,
    ) -> Result<reqwest::Response, VertexError>
    where
        T: serde::Serialize,
    {
        let headers = self.auth_headers(scopes).await?;

        let response = self
            .client
            .post(url)
            .json(&body)
            .headers(headers)
            .send()
            .await
            .map_err(VertexError::ClientError)?;
        Ok(response)
    }

    pub async fn api_get_with_params(
        &self,
        scopes: &[&str],
        url: &str,
        params: Option<Vec<(&str, &str)>>,
    ) -> Result<reqwest::Response, VertexError> {
        let headers = self.auth_headers(scopes).await?;
        let url = match params {
            None => reqwest::Url::parse(url),
            Some(ref query_params) => reqwest::Url::parse_with_params(url, query_params),
        }
        .map_err(|e| VertexError::UrlParseError(e.to_string()))?;

        let response = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(VertexError::ClientError)?;
        Ok(response)
    }

    pub async fn api_get(
        &self,
        scopes: &[&str],
        url: &str,
    ) -> Result<reqwest::Response, VertexError> {
        self.api_get_with_params(scopes, url, None).await
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
    ///     create_advance_site_search: Some(true),
    /// };
    /// let operation = client.create_data_store(request).await?;
    /// ```
    ///
    /// Note: The endpoint URL is built using the project ID, location ("global" by default), and collection name.
    pub async fn create_data_store(
        &self,
        request: CreateDataStoreRequest,
    ) -> Result<Operation, VertexError> {
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
            .api_post(&[BASE_SCOPE], url.unwrap().as_str(), request.data_store)
            .await?
            .error_for_status()
            .map_err(|e| VertexError::HttpStatus(e.to_string()))?;

        let operation: Operation = response.json().await?;

        Ok(operation)
    }
}

pub struct CreateDataStoreRequest {
    pub data_store: DataStore,
    pub project_id: String,
    pub collections: String,
    pub data_store_id: String,
    pub create_advance_site_search: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Operation {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    pub done: bool,
    pub response: HashMap<String, String>,
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
    #[tokio::test]
    async fn test_token_provider() {
        env::set_var(
            "GOOGLE_APPLICATION_CREDENTIALS",
            "../../private/gcp_key.json",
        );
        // load file
        let token_provider = token_provider().await;
        assert!(token_provider.token(&[BASE_SCOPE]).await.is_ok());
        let token = token_provider.token(&[BASE_SCOPE]).await.unwrap();
        assert!(!token.as_str().is_empty());
    }

    // Test create_data_store
    #[tokio::test]
    async fn test_create_data_store() {
        env::set_var(
            "GOOGLE_APPLICATION_CREDENTIALS",
            "../../private/gcp_key.json",
        );

        let project_id = "moni-429523";
        let collections = "default_collection";
        let data_store = DataStore {
            name: "moni-test".to_string(),
            display_name: "moni-test".to_string(),
            industry_vertical: IndustryVertical::Generic,
            solution_types: vec![],
            default_schema_id: None,
            content_config: ContentConfig::PublicWebsite,
            create_time: None,
            language_info: None,
            document_processing_config: None,
            starting_schema: None,
        };

        let data_store_request = CreateDataStoreRequest {
            data_store,
            project_id: project_id.to_string(),
            collections: collections.to_string(),
            data_store_id: "moni-test-9".to_string(),
            create_advance_site_search: None,
        };

        let client = VertexClient::new().await.unwrap();
        let operation = client.create_data_store(data_store_request).await;

        println!("{:?}", operation);

        assert!(operation.is_ok());

        println!("{:?}", operation.unwrap());
    }
}

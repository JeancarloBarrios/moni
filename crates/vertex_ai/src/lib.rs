mod data_store;
pub mod error;
use std::sync::Arc;

use error::VertexError;
use gcp_auth::TokenProvider;
use reqwest::Client;
use tokio::sync::OnceCell;

static TOKEN_PROVIDER: OnceCell<Arc<dyn TokenProvider>> = OnceCell::const_new();

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

    // DataStore Service
}

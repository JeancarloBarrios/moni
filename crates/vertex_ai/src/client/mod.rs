pub mod error;

use std::sync::Arc;

use error::Error;
use gcp_auth::TokenProvider;
use serde_json::Value;
use tokio::sync::OnceCell;

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
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub async fn new() -> Result<Self, Error> {
        let client = reqwest::Client::new();
        Ok(Self { client })
    }

    async fn auth_headers(&self, scopes: &[&str]) -> Result<reqwest::header::HeaderMap, Error> {
        let token_provider = token_provider().await;
        let token = token_provider
            .token(scopes)
            .await
            .map_err(Error::ProviderError)?;
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
    ) -> Result<reqwest::Response, Error>
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
            .map_err(Error::ClientError)?;
        Ok(response)
    }

    pub async fn api_get_with_params(
        &self,
        scopes: &[&str],
        url: &str,
        params: Option<Vec<(&str, &str)>>,
    ) -> Result<reqwest::Response, Error> {
        let headers = self.auth_headers(scopes).await?;
        let url = match params {
            None => reqwest::Url::parse(url),
            Some(ref query_params) => reqwest::Url::parse_with_params(url, query_params),
        }
        .map_err(|e| Error::UrlParseError(e.to_string()))?;

        let response = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(Error::ClientError)?;
        Ok(response)
    }

    pub async fn api_get(&self, scopes: &[&str], url: &str) -> Result<reqwest::Response, Error> {
        self.api_get_with_params(scopes, url, None).await
    }

    pub async fn api_delete(
        &self,
        scopes: &[&str],
        url: &str,
        params: Option<Vec<(&str, &str)>>,
    ) -> Result<reqwest::Response, Error> {
        let headers = self.auth_headers(scopes).await?;
        let url = match params {
            None => reqwest::Url::parse(url),
            Some(ref query_params) => reqwest::Url::parse_with_params(url, query_params),
        }
        .map_err(|e| Error::UrlParseError(e.to_string()))?;

        let response = self
            .client
            .delete(url)
            .headers(headers)
            .send()
            .await
            .map_err(Error::ClientError)?;
        Ok(response)
    }
}

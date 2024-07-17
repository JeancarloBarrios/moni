use thiserror::Error;

#[derive(Debug, Error)]
pub enum VertexError {
    #[error("provider error")]
    ProviderError(gcp_auth::Error),

    #[error("client error")]
    ClientError(reqwest::Error),

    #[error("url parsing error reason: {0}")]
    UrlParseError(String),
}

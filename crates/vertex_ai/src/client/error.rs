#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("provider error")]
    ProviderError(gcp_auth::Error),

    #[error("client error")]
    ClientError(reqwest::Error),

    #[error("url parsing error reason: {0}")]
    UrlParseError(String),

    #[error("HTTP status error: {0}")]
    HttpStatus(String),

    #[error("JSON parsing error")]
    ResponseJsonParsing(#[from] reqwest::Error),
}

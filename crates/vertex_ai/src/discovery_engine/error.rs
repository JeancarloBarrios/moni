#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("client error")]
    ClientError(crate::client::error::Error),

    #[error("HTTP status error")]
    HttpStatus(reqwest::Error),

    #[error("some random datastore error")]
    DataStoreError,

    #[error("JSON parsing error")]
    ResponseJsonParsing(#[from] reqwest::Error),

    #[error("Text response error")]
    ResponseTextRetrieval(reqwest::Error),
}

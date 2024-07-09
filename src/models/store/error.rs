use firebase_rs::UrlParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("Unable to connect to store")]
    Connection(#[from] UrlParseError),
}

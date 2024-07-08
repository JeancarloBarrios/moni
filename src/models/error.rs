use thiserror::Error;

use super::store::error::StoreError;

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("invalid cofiguration")]
    InvalidConfiguration,

    #[error("store error")]
    StoreError(#[from] StoreError),
}

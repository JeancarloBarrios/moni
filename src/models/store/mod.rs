pub mod error;

use error::StoreError;

use firebase_rs::Firebase;

pub struct Store {
    pub key: String,
    pub url: String,
}

type Db = Firebase;

impl Store {
    pub fn with_key_url(key: &str, url: &str) -> Result<Self, StoreError> {
        let _ = firebase_rs::Firebase::new(url).map_err(error::StoreError::Connection)?;
        Ok(Self {
            key: key.to_string(),
            url: url.to_string(),
        })
    }

    pub fn db(&self) -> Result<Db, StoreError> {
        firebase_rs::Firebase::new(&self.url).map_err(error::StoreError::Connection)
    }
}

use serde::{Deserialize, Serialize};

use super::{error::ModelError, ModuleManager};

const DOCUMENT_TABLE: &str = "documents";

#[derive(Deserialize, Serialize, Debug)]
pub struct Document {
    pub tittle: String,
    pub name: String,
}

pub struct DocumentCtrl {}

impl DocumentCtrl {
    fn new() -> Self {
        Self {}
    }

    async fn create_documet(self, mm: ModuleManager, document: Document) -> Result<(), ModelError> {
        let db = mm.store.db().map_err(ModelError::StoreError)?;
        let _ = db.at(DOCUMENT_TABLE).set(&document).await;
        Ok(())
    }

    async fn get_documents(self, mm: ModuleManager) -> Result<Vec<Document>, ModelError> {
        let db = mm.store.db().map_err(ModelError::StoreError)?;
        db.at(DOCUMENT_TABLE)
            .get::<Vec<Document>>()
            .await
            .map_err(ModelError::RequestError)
    }
}

use serde::{Deserialize, Serialize};

use super::{error::ModelError, ModuleManager};

const DOCUMENT_TABLE: &str = "documents";

#[derive(Deserialize, Serialize, Debug)]
pub struct Document {
    pub tittle: String,
    pub name: String,
}

pub struct DocumentCtrl {
    mm: ModuleManager,
}

impl DocumentCtrl {
    fn new(mm: ModuleManager) -> Self {
        Self { mm }
    }

    async fn create_documet(self, document: Document) -> Result<(), ModelError> {
        let db = self.mm.store.db().map_err(ModelError::StoreError)?;
        let _ = db.at(DOCUMENT_TABLE).set(&document).await;
        Ok(())
    }
}

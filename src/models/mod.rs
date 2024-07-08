pub mod error;
pub(crate) mod store;

use error::ModelError;
use store::Store;

pub struct ModuleManager {
    pub store: Store,
}

pub struct ModuleManagerBuilder {
    pub fire_store_config: Option<Store>,
}

impl ModuleManagerBuilder {
    pub fn new() -> Self {
        ModuleManagerBuilder {
            fire_store_config: None,
        }
    }
    pub fn fire_store_config(mut self, config: Store) -> Self {
        self.fire_store_config = Some(config);
        self
    }

    pub fn build(self) -> Result<ModuleManager, ModelError> {
        let store_config = self
            .fire_store_config
            .ok_or(ModelError::InvalidConfiguration)?;
        let store = Store::with_key_url(&store_config.key, &store_config.url)
            .map_err(ModelError::StoreError)?;
        Ok(ModuleManager { store })
    }
}

impl ModuleManager {
    fn builder() -> ModuleManagerBuilder {
        ModuleManagerBuilder::new()
    }
}

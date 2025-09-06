use crate::config::ModelConfig;

// Maintains a registry of available models
pub struct ModelRegistry {
    models: Vec<ModelConfig>,
}

impl ModelRegistry {
    pub fn new(models: Vec<ModelConfig>) -> Self {
        Self { models }
    }

    pub fn list_models(&self) -> &Vec<ModelConfig> {
        &self.models
    }
}
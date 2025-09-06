use crate::config::ModelConfig;

// Manages model lifecycle
pub struct ModelManager {
    models: Vec<ModelConfig>,
}

impl ModelManager {
    pub fn new(models: Vec<ModelConfig>) -> Self {
        Self { models }
    }

    pub fn get_model(&self, id: &str) -> Option<&ModelConfig> {
        self.models.iter().find(|m| m.id == id)
    }
}   
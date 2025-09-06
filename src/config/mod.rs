use serde::Deserialize;

// Model configuration for Nexus
#[derive(Clone, Debug, Deserialize)]
pub struct ModelConfig {
    pub id: String,
    pub endpoint: String,
    pub max_requests_per_second: u32,
    pub weight: f64,
}

impl ModelConfig {
    pub fn new(id: &str, endpoint: &str, max_requests_per_second: u32, weight: f64) -> Self {
        Self {
            id: id.to_string(),
            endpoint: endpoint.to_string(),
            max_requests_per_second,
            weight,
        }
    }
}
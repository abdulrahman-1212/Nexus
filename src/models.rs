use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Request structure for inference
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InferenceRequest {
    pub id: String,
    pub model_id: String,
    pub input: String,
}

impl InferenceRequest {
    pub fn new(model_id: &str, input: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            model_id: model_id.to_string(),
            input: input.to_string(),
        }
    }
}

// Response structure for inference
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InferenceResponse {
    pub id: String,
    pub output: String,
    pub latency_ms: u64,
}
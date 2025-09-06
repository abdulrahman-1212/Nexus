use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// Model context for carrying session and metadata
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelContext {
    pub session_id: String,
    pub metadata: HashMap<String, String>,
}

// Request structure for inference
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InferenceRequest {
    pub id: String,
    pub model_id: String,
    pub input: String,
    pub context: ModelContext,
}

impl InferenceRequest {
    pub fn new(model_id: &str, input: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            model_id: model_id.to_string(),
            input: input.to_string(),
            context: ModelContext {
                session_id: Uuid::new_v4().to_string(),
                metadata: HashMap::new(),
            },
        }
    }
}

// Response structure for inference
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InferenceResponse {
    pub id: String,
    pub output: String,
    pub latency_ms: u64,
    pub context: ModelContext,
}
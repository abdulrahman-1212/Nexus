use crate::{config::ModelConfig, models::{InferenceRequest, InferenceResponse}};
use tokio::time::{self, Duration};

// Executes model inference
pub struct ModelRunner;

impl ModelRunner {
    pub fn new() -> Self {
        Self
    }

    pub async fn run_inference(&self, request: &InferenceRequest, model: &ModelConfig) -> InferenceResponse {
        // Simulate inference
        time::sleep(Duration::from_millis(100)).await;
        let context_info = if !request.context.metadata.is_empty() {
            format!(" with context metadata: {:?}", request.context.metadata)
        } else {
            String::new()
        };
        InferenceResponse {
            id: request.id.clone(),
            output: format!("Processed by {}: {}{} (session: {})", model.id, request.input, context_info, request.context.session_id),
            latency_ms: 100,
            context: request.context.clone(),
        }
    }
}
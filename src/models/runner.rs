use crate::{config::ModelConfig, models::{InferenceRequest, InferenceResponse}};
use reqwest::Client;

// Executes model inference
#[derive(Clone)]
pub struct ModelRunner {
    client: Client,
}

impl ModelRunner {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn run_inference(&self, request: &InferenceRequest, model: &ModelConfig) -> InferenceResponse {
        let context_info = if !request.context.metadata.is_empty() {
            format!("Metadata: {:?}", request.context.metadata)
        } else {
            String::new()
        };
        let prompt = format!(
            "Session: {}, {}, Input: {}",
            request.context.session_id, context_info, request.input
        );

        let payload = serde_json::json!({
            "model": &model.id,
            "prompt": prompt,
            "stream": false,
            "options": {
                "temperature": 0.7,
                "max_tokens": 100
            }
        });

        match self.client
            .post(&model.endpoint)
            .json(&payload)
            .send()
            .await
        {
            Ok(resp) => match resp.json::<serde_json::Value>().await {
                Ok(json) => {
                    let output = json["response"]
                        .as_str()
                        .unwrap_or("Error: No response field")
                        .to_string();
                    InferenceResponse {
                        id: request.id.clone(),
                        output,
                        latency_ms: 100, // Placeholder; actual latency measured in orchestrator
                        context: request.context.clone(),
                    }
                }
                Err(e) => InferenceResponse {
                    id: request.id.clone(),
                    output: format!("Error parsing response: {}", e),
                    latency_ms: 100,
                    context: request.context.clone(),
                },
            },
            Err(e) => InferenceResponse {
                id: request.id.clone(),
                output: format!("Error calling Ollama: {}", e),
                latency_ms: 100,
                context: request.context.clone(),
            },
        }
    }
}
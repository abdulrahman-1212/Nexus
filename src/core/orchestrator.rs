use crate::{config::ModelConfig, core::{batcher::Batcher, cache::Cache, load_balancer::LoadBalancer}, models::{InferenceRequest, InferenceResponse}, observability::metrics::Metrics};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use reqwest::Client;
use tokio::time::Duration;

// Core orchestrator for managing inference requests
#[derive(Clone)]
pub struct NexusOrchestrator {
    load_balancer: LoadBalancer,
    batcher: Batcher,
    cache: Cache,
    metrics: Arc<Mutex<Metrics>>,
    client: Client,
}

impl NexusOrchestrator {
    pub fn new(models: Vec<ModelConfig>) -> Self {
        Self {
            load_balancer: LoadBalancer::new(models),
            batcher: Batcher::new(),
            cache: Cache::new(),
            metrics: Metrics::new(),
            client: Client::new(),
        }
    }

    pub async fn process_batch(&self, batch: Vec<InferenceRequest>) -> Vec<InferenceResponse> {
        let mut responses = Vec::new();
        for request in batch {
            // Check cache first
            if let Some(cached_response) = self.cache.get(&request.id) {
                self.metrics.lock().unwrap().record_cache_hit();
                responses.push(cached_response);
                continue;
            }

            let model = match self.load_balancer.select_model() {
                Some(model) => model,
                None => {
                    self.metrics.lock().unwrap().record_error();
                    responses.push(InferenceResponse {
                        id: request.id.clone(),
                        output: "No available models".to_string(),
                        latency_ms: 0,
                        context: request.context.clone(),
                    });
                    continue;
                }
            };

            self.load_balancer.increment_request_count(&model.id);
            let start_time = Instant::now();

            let response = self.run_inference(&request, model).await;

            let latency_ms = start_time.elapsed().as_millis() as u64;
            self.metrics.lock().unwrap().record_request(latency_ms);

            // Cache the response
            self.cache.set(request.id.clone(), response.clone());

            self.load_balancer.decrement_request_count(&model.id);
            responses.push(response);
        }
        responses
    }

    async fn run_inference(&self, request: &InferenceRequest, model: &ModelConfig) -> InferenceResponse {
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
                        latency_ms: Instant::now().elapsed().as_millis() as u64,
                        context: request.context.clone(),
                    }
                }
                Err(e) => InferenceResponse {
                    id: request.id.clone(),
                    output: format!("Error parsing response: {}", e),
                    latency_ms: Instant::now().elapsed().as_millis() as u64,
                    context: request.context.clone(),
                },
            },
            Err(e) => InferenceResponse {
                id: request.id.clone(),
                output: format!("Error calling Ollama: {}", e),
                latency_ms: Instant::now().elapsed().as_millis() as u64,
                context: request.context.clone(),
            },
        }
    }

    pub fn get_metrics(&self) -> Metrics {
        self.metrics.lock().unwrap().clone()
    }
}
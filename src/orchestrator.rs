use crate::{cache::Cache, config::ModelConfig, load_balancer::LoadBalancer, metrics::Metrics, models::{InferenceRequest, InferenceResponse}};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::time::{self, Duration};

// Core orchestrator for managing inference requests
#[derive(Clone)]
pub struct NexusOrchestrator {
    load_balancer: LoadBalancer,
    cache: Cache,
    metrics: Arc<Mutex<Metrics>>,
}

impl NexusOrchestrator {
    pub fn new(models: Vec<ModelConfig>) -> Self {
        Self {
            load_balancer: LoadBalancer::new(models),
            cache: Cache::new(),
            metrics: Metrics::new(),
        }
    }

    pub async fn process_request(&self, request: InferenceRequest) -> InferenceResponse {
        // Check cache first
        if let Some(cached_response) = self.cache.get(&request.id) {
            self.metrics.lock().unwrap().record_cache_hit();
            return cached_response;
        }

        // Select model using load balancer
        let model = match self.load_balancer.select_model() {
            Some(model) => model,
            None => {
                self.metrics.lock().unwrap().record_error();
                return InferenceResponse {
                    id: request.id,
                    output: "No available models".to_string(),
                    latency_ms: 0,
                };
            }
        };

        self.load_balancer.increment_request_count(&model.id);
        let start_time = Instant::now();

        // Simulate model inference (replace with actual model endpoint call)
        let response = self.simulate_inference(&request, model).await;

        // Update metrics
        let latency_ms = start_time.elapsed().as_millis() as u64;
        self.metrics.lock().unwrap().record_request(latency_ms);

        // Cache the response
        self.cache.set(request.id.clone(), response.clone());

        self.load_balancer.decrement_request_count(&model.id);
        response
    }

    async fn simulate_inference(&self, request: &InferenceRequest, model: &ModelConfig) -> InferenceResponse {
        // Simulate network delay
        time::sleep(Duration::from_millis(100)).await;
        
        InferenceResponse {
            id: request.id.clone(),
            output: format!("Processed by {}: {}", model.id, request.input),
            latency_ms: 100,
        }
    }

    pub fn get_metrics(&self) -> Metrics {
        self.metrics.lock().unwrap().clone()
    }
}
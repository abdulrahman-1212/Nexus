use crate::config::ModelConfig;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Load balancer for distributing requests across models
#[derive(Clone)]
pub struct LoadBalancer {
    models: Vec<ModelConfig>,
    request_counts: Arc<Mutex<HashMap<String, u32>>>,
}

impl LoadBalancer {
    pub fn new(models: Vec<ModelConfig>) -> Self {
        let request_counts = Arc::new(Mutex::new(HashMap::new()));
        for model in &models {
            request_counts.lock().unwrap().insert(model.id.clone(), 0);
        }
        Self { models, request_counts }
    }

    // Select model based on A/B testing weights and current load
    pub fn select_model(&self) -> Option<&ModelConfig> {
        let mut rng = rand::thread_rng();
        let total_weight: f64 = self.models.iter().map(|m| m.weight).sum();
        let mut weight = rng.gen::<f64>() * total_weight;

        for model in &self.models {
            weight -= model.weight;
            if weight <= 0.0 {
                let counts = self.request_counts.lock().unwrap();
                if counts.get(&model.id).unwrap_or(&0) < &model.max_requests_per_second {
                    return Some(model);
                }
            }
        }
        // Fallback to least loaded model
        let counts = self.request_counts.lock().unwrap();
        self.models.iter()
            .filter(|model| counts.get(&model.id).unwrap_or(&0) < &model.max_requests_per_second)
            .min_by_key(|model| counts.get(&model.id).unwrap_or(&0))
    }

    pub fn increment_request_count(&self, model_id: &str) {
        let mut counts = self.request_counts.lock().unwrap();
        *counts.entry(model_id.to_string()).or_insert(0) += 1;
    }

    pub fn decrement_request_count(&self, model_id: &str) {
        let mut counts = self.request_counts.lock().unwrap();
        if let Some(count) = counts.get_mut(model_id) {
            *count = count.saturating_sub(1);
        }
    }
}
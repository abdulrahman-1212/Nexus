use serde::Serialize;
use std::sync::{Arc, Mutex};

// Metrics for observability
#[derive(Clone, Debug, Default, Serialize)]
pub struct Metrics {
    pub requests_processed: u64,
    pub average_latency_ms: f64,
    pub error_count: u64,
    pub cache_hits: u64,
}

impl Metrics {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self::default()))
    }

    pub fn record_request(&self, latency_ms: u64) {
        let mut metrics = self.clone();
        metrics.requests_processed += 1;
        metrics.average_latency_ms =
            (metrics.average_latency_ms * (metrics.requests_processed - 1) as f64 + latency_ms as f64)
            / metrics.requests_processed as f64;
    }

    pub fn record_error(&self) {
        let mut metrics = self.clone();
        metrics.error_count += 1;
    }

    pub fn record_cache_hit(&self) {
        let mut metrics = self.clone();
        metrics.cache_hits += 1;
    }
}
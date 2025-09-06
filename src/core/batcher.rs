use crate::models::InferenceRequest;
use tokio::time::{self, Duration};

// Request batcher for grouping inference requests
#[derive(Clone)]
pub struct Batcher {
    batch_size: usize,
    batch_timeout: Duration,
}

impl Batcher {
    pub fn new() -> Self {
        Self {
            batch_size: 10,
            batch_timeout: Duration::from_millis(50),
        }
    }

    pub async fn add_request(&self, request: InferenceRequest) -> Vec<InferenceRequest> {
        let mut batch = vec![request];
        let start = time::Instant::now();
        while batch.len() < self.batch_size && start.elapsed() < self.batch_timeout {
            // Placeholder for collecting more requests; currently simulating single-request batch
            time::sleep(Duration::from_millis(10)).await;
        }
        batch
    }
}
use crate::models::InferenceResponse;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// In-memory cache for inference responses
#[derive(Clone)]
pub struct Cache {
    store: Arc<Mutex<HashMap<String, InferenceResponse>>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get(&self, request_id: &str) -> Option<InferenceResponse> {
        self.store.lock().unwrap().get(request_id).cloned()
    }

    pub fn set(&self, request_id: String, response: InferenceResponse) {
        self.store.lock().unwrap().insert(request_id, response);
    }
}
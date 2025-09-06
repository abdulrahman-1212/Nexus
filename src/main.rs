mod config;
mod load_balancer;
mod cache;
mod metrics;
mod models;
mod orchestrator;
mod server;

use hyper::{Server, service::{make_service_fn, service_fn}};
use config::ModelConfig;
use models::InferenceRequest;
use orchestrator::NexusOrchestrator;
use server::handle_request;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{self, Duration};

// Entry point for the Nexus LLM inference orchestration platform
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize model configurations
    let models = vec![
        ModelConfig {
            id: "model1".to_string(),
            endpoint: "http://localhost:8000".to_string(),
            max_requests_per_second: 10,
            weight: 0.7, // 70% of requests for A/B testing
        },
        ModelConfig {
            id: "model2".to_string(),
            endpoint: "http://localhost:8001".to_string(),
            max_requests_per_second: 10,
            weight: 0.3, // 30% of requests for A/B testing
        },
    ];

    // Create orchestrator
    let orchestrator = Arc::new(NexusOrchestrator::new(models));

    // Spawn metrics printer
    let metrics_orchestrator = orchestrator.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            println!("Metrics: {:?}", metrics_orchestrator.get_metrics());
        }
    });

    // Create a channel for request batching
    let (tx, mut rx) = mpsc::channel::<InferenceRequest>(100);

    // Spawn request batch processor
    let batch_orchestrator = orchestrator.clone();
    tokio::spawn(async move {
        while let Some(request) = rx.recv().await {
            let orchestrator = batch_orchestrator.clone();
            tokio::spawn(async move {
                let response = orchestrator.process_request(request).await;
                println!("Response: {:?}", response);
            });
        }
    });

    // Start HTTP server
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_svc = make_service_fn(move |_conn| {
        let orchestrator = orchestrator.clone();
        let tx = tx.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| handle_request(req, orchestrator.clone(), tx.clone())))
        }
    });

    println!("Starting Nexus server on http://{}", addr);
    Server::bind(&addr)
        .serve(make_svc)
        .await?;

    Ok(())
}
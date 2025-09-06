mod api;
mod config;
mod core;
mod models;
mod observability;

use hyper::{Server, service::{make_service_fn, service_fn}};
use config::ModelConfig;
use core::{batcher::Batcher, orchestrator::NexusOrchestrator};
use api::inference::handle_inference;
use api::metrics::handle_metrics;
use observability::health::handle_health;
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
            weight: 0.7,
        },
        ModelConfig {
            id: "model2".to_string(),
            endpoint: "http://localhost:8001".to_string(),
            max_requests_per_second: 10,
            weight: 0.3,
        },
    ];

    // Create orchestrator
    let orchestrator = Arc::new(NexusOrchestrator::new(models));
    let batcher = Batcher::new();

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
    let (tx, mut rx) = mpsc::channel(100);

    // Spawn request batch processor
    let batch_orchestrator = orchestrator.clone();
    tokio::spawn(async move {
        let batcher = batcher.clone();
        while let Some(request) = rx.recv().await {
            let batch = batcher.add_request(request).await;
            let orchestrator = batch_orchestrator.clone();
            tokio::spawn(async move {
                let responses = orchestrator.process_batch(batch).await;
                for response in responses {
                    println!("Response: {:?}", response);
                }
            });
        }
    });

    // Start HTTP server
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_svc = make_service_fn(move |_conn| {
        let orchestrator = orchestrator.clone();
        let tx = tx.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let orchestrator = orchestrator.clone();
                let tx = tx.clone();
                async move {
                    match (req.method(), req.uri().path()) {
                        (&hyper::Method::POST, "/infer") => handle_inference(req, orchestrator, tx).await,
                        (&hyper::Method::GET, "/metrics") => handle_metrics(req, orchestrator).await,
                        (&hyper::Method::GET, "/health") => handle_health(req, orchestrator).await,
                        _ => Ok(hyper::Response::builder()
                            .status(hyper::StatusCode::NOT_FOUND)
                            .body(hyper::Body::from("Not found"))
                            .unwrap()),
                    }
                }
            }))
        }
    });

    println!("Starting Nexus server on http://{}", addr);
    Server::bind(&addr)
        .serve(make_svc)
        .await?;

    Ok(())
}
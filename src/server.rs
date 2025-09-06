use crate::{models::InferenceRequest, orchestrator::NexusOrchestrator};
use hyper::{Body, Request, Response, StatusCode};
use serde_json::to_string;
use std::sync::Arc;
use tokio::sync::mpsc;

// Handle HTTP requests for inference
pub async fn handle_request(
    req: Request<Body>,
    orchestrator: Arc<NexusOrchestrator>,
    tx: mpsc::Sender<InferenceRequest>,
) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&hyper::Method::POST, "/infer") => {
            let body_bytes = hyper::body::to_bytes(req.into_body()).await?;
            let request: InferenceRequest = match serde_json::from_slice(&body_bytes) {
                Ok(req) => req,
                Err(_) => {
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Invalid request format"))
                        .unwrap());
                }
            };

            // Send request to orchestrator via channel
            tx.send(request.clone()).await.unwrap();
            let response = orchestrator.process_request(request).await;

            Ok(Response::new(Body::from(to_string(&response).unwrap())))
        }
        (&hyper::Method::GET, "/metrics") => {
            let metrics = orchestrator.get_metrics();
            Ok(Response::new(Body::from(to_string(&metrics).unwrap())))
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not found"))
            .unwrap()),
    }
}
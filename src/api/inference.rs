use crate::{models::InferenceRequest, core::orchestrator::NexusOrchestrator};
use hyper::{Body, Request, Response, StatusCode};
use std::sync::Arc;
use tokio::sync::mpsc;

pub async fn handle_inference(
    req: Request<Body>,
    _orchestrator: Arc<NexusOrchestrator>,
    tx: mpsc::Sender<InferenceRequest>,
) -> Result<Response<Body>, hyper::Error> {
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

    tx.send(request).await.unwrap();
    Ok(Response::new(Body::from("Request submitted")))
}
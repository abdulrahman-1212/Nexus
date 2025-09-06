use crate::core::orchestrator::NexusOrchestrator;
use hyper::{Body, Request, Response, StatusCode};
use std::sync::Arc;

pub async fn handle_health(_req: Request<Body>, _orchestrator: Arc<NexusOrchestrator>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Healthy"))
        .unwrap())
}
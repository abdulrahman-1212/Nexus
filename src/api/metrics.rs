use crate::core::orchestrator::NexusOrchestrator;
use hyper::{Body, Request, Response};
use serde_json::to_string;
use std::sync::Arc;

pub async fn handle_metrics(
    _req: Request<Body>,
    orchestrator: Arc<NexusOrchestrator>,
) -> Result<Response<Body>, hyper::Error> {
    let metrics = orchestrator.get_metrics();
    Ok(Response::new(Body::from(to_string(&metrics).unwrap())))
}
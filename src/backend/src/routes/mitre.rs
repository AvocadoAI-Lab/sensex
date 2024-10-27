use axum::{
    Router,
    routing::post,
};
use crate::handlers::mitre::*;

pub fn routes() -> Router {
    Router::new()
        .route("/mitre/metadata", post(get_mitre_metadata))
        .route("/mitre/references", post(get_mitre_references))
        .route("/mitre/techniques", post(get_mitre_techniques))
        .route("/mitre/tactics", post(get_mitre_tactics))
}

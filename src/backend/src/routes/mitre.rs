use axum::{
    Router,
    routing::get,
};
use crate::handlers::mitre::*;

pub fn routes() -> Router {
    Router::new()
        .route("/mitre/metadata", get(get_mitre_metadata))
        .route("/mitre/references", get(get_mitre_references))
        .route("/mitre/techniques", get(get_mitre_techniques))
        .route("/mitre/tactics", get(get_mitre_tactics))
        .route("/mitre/groups", get(get_mitre_groups))
        .route("/mitre/mitigations", get(get_mitre_mitigations))
        .route("/mitre/software", get(get_mitre_software))
}

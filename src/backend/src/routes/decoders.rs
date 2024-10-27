use axum::{
    Router,
    routing::{get, put, delete},
};
use crate::handlers::decoders::*;

pub fn routes() -> Router {
    Router::new()
        // Base decoders endpoint
        .route("/decoders", get(get_decoders))
        
        // Decoder files operations
        .route("/decoders/files", get(get_decoder_files))
        .route("/decoders/files/:filename", get(get_decoder_file_content))
        .route("/decoders/files/:filename", put(update_decoder_file))
        .route("/decoders/files/:filename", delete(delete_decoder_file))
}

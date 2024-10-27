use axum::Json;
use serde::Deserialize;
use crate::client::WazuhClient;

#[derive(Debug, Deserialize)]
pub struct WazuhRequest {
    pub endpoint: String,
    pub token: String,
}

pub async fn handle_wazuh_request(request: WazuhRequest, url_path: &str, handler: impl FnOnce(String) -> String) -> Json<serde_json::Value> {
    let client = WazuhClient::new();
    let url = handler(format!("{}/{}", request.endpoint, url_path));
    
    match client.get(&url, Some(&request.token)).await {
        Ok(response) => {
            match WazuhClient::handle_json_response(response).await {
                Ok(data) => Json(data),
                Err(e) => Json(serde_json::json!({
                    "error": e
                })),
            }
        },
        Err(e) => Json(serde_json::json!({
            "error": format!("Request failed: {}", e)
        })),
    }
}

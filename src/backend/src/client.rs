use reqwest::{Client, Response};
use serde_json::Value;

pub struct WazuhClient {
    client: Client,
}

impl WazuhClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        
        Self { client }
    }

    pub async fn get(&self, url: &str, token: Option<&str>) -> Result<Response, reqwest::Error> {
        let mut request = self.client.get(url);
        
        if let Some(token) = token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        
        request.send().await
    }

    pub async fn get_with_auth(&self, url: &str, username: &str, password: &str) -> Result<Response, reqwest::Error> {
        self.client
            .get(url)
            .basic_auth(username, Some(password))
            .send()
            .await
    }

    pub async fn handle_json_response(response: Response) -> Result<Value, String> {
        match response.json::<Value>().await {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("Failed to parse response: {}", e)),
        }
    }
}

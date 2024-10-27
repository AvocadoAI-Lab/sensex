use reqwest::{Client, Response};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct CacheEntry {
    data: Value,
    timestamp: Instant,
}

#[derive(Clone)]
pub struct WazuhClient {
    client: Client,
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    cache_duration: Duration,
}

impl WazuhClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        
        Self { 
            client,
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_duration: Duration::from_secs(300), // 5 minutes default cache
        }
    }

    pub async fn get_cached(&self, url: &str, token: Option<&str>) -> Result<Value, String> {
        let cache_key = match token {
            Some(t) => format!("{}:{}", url, t),
            None => url.to_string(),
        };

        // Try to get from cache first
        if let Some(cached_data) = self.get_from_cache(&cache_key).await {
            return Ok(cached_data);
        }

        // If not in cache, make the request
        let response = self.get(url, token).await
            .map_err(|e| format!("Request failed: {}", e))?;

        let data = Self::handle_json_response(response).await?;
        
        // Store in cache
        self.store_in_cache(&cache_key, data.clone()).await;
        
        Ok(data)
    }

    async fn get_from_cache(&self, key: &str) -> Option<Value> {
        let cache = self.cache.read().await;
        if let Some(entry) = cache.get(key) {
            if entry.timestamp.elapsed() < self.cache_duration {
                return Some(entry.data.clone());
            }
        }
        None
    }

    async fn store_in_cache(&self, key: &str, data: Value) {
        let mut cache = self.cache.write().await;
        cache.insert(key.to_string(), CacheEntry {
            data,
            timestamp: Instant::now(),
        });
    }

    pub async fn get(&self, url: &str, token: Option<&str>) -> Result<Response, reqwest::Error> {
        let mut request = self.client.get(url);
        
        if let Some(token) = token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        
        request.send().await
    }

    pub async fn post(&self, url: &str, token: Option<&str>, body: Option<Value>) -> Result<Response, reqwest::Error> {
        let mut request = self.client.post(url);
        
        if let Some(token) = token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        if let Some(json_body) = body {
            request = request.json(&json_body);
        }
        
        request.send().await
    }

    pub async fn put(&self, url: &str, token: Option<&str>, body: Option<Value>) -> Result<Response, reqwest::Error> {
        let mut request = self.client.put(url);
        
        if let Some(token) = token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        if let Some(json_body) = body {
            request = request.json(&json_body);
        }
        
        request.send().await
    }

    pub async fn delete(&self, url: &str, token: Option<&str>) -> Result<Response, reqwest::Error> {
        let mut request = self.client.delete(url);
        
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

    pub fn set_cache_duration(&mut self, duration: Duration) {
        self.cache_duration = duration;
    }

    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }

    pub async fn get_auth_token(&self) -> Result<String, String> {
        let response = self
            .get_with_auth(
                "https://wazuh.aixsoar.com:55000/security/user/authenticate",
                "wazuh-wui",
                "S.Ouv.51BHmQ*wqhq0O?eKSAyshu0Z.*"
            )
            .await
            .map_err(|e| format!("Authentication request failed: {}", e))?;

        let json = Self::handle_json_response(response).await?;
        
        json.get("data")
            .and_then(|data| data.get("token"))
            .and_then(|token| token.as_str())
            .map(String::from)
            .ok_or_else(|| "Token not found in response".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_URL: &str = "https://wazuh.aixsoar.com:55000/security/user/authenticate";
    const TEST_USERNAME: &str = "wazuh-wui";
    const TEST_PASSWORD: &str = "S.Ouv.51BHmQ*wqhq0O?eKSAyshu0Z.*";
    const INVALID_PASSWORD: &str = "wrong_password";

    #[tokio::test]
    async fn test_successful_authentication() {
        let client = WazuhClient::new();
        
        let response = client
            .get_with_auth(TEST_URL, TEST_USERNAME, TEST_PASSWORD)
            .await;
            
        assert!(response.is_ok(), "Authentication should succeed with valid credentials");
        
        if let Ok(resp) = response {
            assert_eq!(resp.status().as_u16(), 200, "Should receive 200 OK status");
            
            let json_result = WazuhClient::handle_json_response(resp).await;
            assert!(json_result.is_ok(), "Should be able to parse JSON response");
        }
    }

    #[tokio::test]
    async fn test_failed_authentication() {
        let client = WazuhClient::new();
        
        let response = client
            .get_with_auth(TEST_URL, TEST_USERNAME, INVALID_PASSWORD)
            .await;
            
        assert!(response.is_ok(), "Request should complete even with invalid credentials");
        
        if let Ok(resp) = response {
            assert_eq!(resp.status().as_u16(), 401, "Should receive 401 Unauthorized status");
        }
    }

    #[tokio::test]
    async fn test_invalid_url() {
        let client = WazuhClient::new();
        let invalid_url = "https://invalid.example.com:55000";
        
        let response = client
            .get_with_auth(invalid_url, TEST_USERNAME, TEST_PASSWORD)
            .await;
            
        assert!(response.is_err(), "Request should fail with invalid URL");
    }

    #[tokio::test]
    async fn test_cache_with_auth() {
        let client = WazuhClient::new();
        let test_endpoint = format!("{}/security/user/authenticate", TEST_URL);
        
        // First request should hit the API
        let first_response = client
            .get_cached(&test_endpoint, Some(TEST_PASSWORD))
            .await;
        assert!(first_response.is_ok(), "First request should succeed");
        
        // Second request should come from cache
        let second_response = client
            .get_cached(&test_endpoint, Some(TEST_PASSWORD))
            .await;
        assert!(second_response.is_ok(), "Second request should succeed");
        
        assert_eq!(
            first_response.unwrap(),
            second_response.unwrap(),
            "Cached response should match original response"
        );
    }

    #[tokio::test]
    async fn test_clear_cache() {
        let client = WazuhClient::new();
        let test_endpoint = format!("{}/security/user/authenticate", TEST_URL);
        
        // First request
        let first_response = client
            .get_cached(&test_endpoint, Some(TEST_PASSWORD))
            .await;
        assert!(first_response.is_ok(), "First request should succeed");
        
        // Clear cache
        client.clear_cache().await;
        
        // Second request should hit API again
        let second_response = client
            .get_cached(&test_endpoint, Some(TEST_PASSWORD))
            .await;
        assert!(second_response.is_ok(), "Second request should succeed after cache clear");
    }
}

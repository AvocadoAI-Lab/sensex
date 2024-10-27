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
}

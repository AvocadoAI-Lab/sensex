use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::Value;
use std::error::Error;
use super::common::{WAZUH_URL, PROXY_URL, get_test_client};
use super::test_utils::{TestEndpoint, test_endpoint, setup_test_directory};

pub struct TestFramework {
    pub client: Client,
    pub headers: HeaderMap,
    pub module_name: String,
    pub base_request: Value,
    proxy_url: String,
}

impl TestFramework {
    pub async fn new(module_name: &str) -> Result<Self, Box<dyn Error>> {
        // 設置測試目錄
        setup_test_directory(module_name)?;

        // 獲取認證 token
        let (_, token) = get_test_client().await;
        
        // 創建 HTTP client
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        // 創建 headers
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", token))?);

        // 基本請求結構
        let base_request = serde_json::json!({
            "endpoint": WAZUH_URL,
            "token": token,
            "url": WAZUH_URL
        });

        Ok(Self {
            client,
            headers,
            module_name: module_name.to_string(),
            base_request,
            proxy_url: PROXY_URL.to_string(),
        })
    }

    pub async fn test_endpoint(&self, endpoint: TestEndpoint) -> Result<Value, Box<dyn Error>> {
        let max_retries = 3;
        let mut last_error = None;

        for attempt in 1..=max_retries {
            match test_endpoint(
                &self.client,
                &self.headers,
                endpoint.clone(),
                &self.proxy_url,
                &self.module_name
            ).await {
                Ok(json_value) => return Ok(json_value),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < max_retries {
                        println!("Attempt {} failed, retrying...", attempt);
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    }
                }
            }
        }

        // 如果所有重試都失敗了，返回錯誤
        Err(last_error.unwrap())
    }

    // Helper method to create a basic endpoint
    pub fn create_endpoint(&self, path: &str) -> TestEndpoint {
        TestEndpoint::new(path, None, Some(self.base_request.clone()))
    }

    // Helper method to create an endpoint with parameters
    pub fn create_endpoint_with_params(&self, path: &str, param_desc: &str, params: Value) -> TestEndpoint {
        let mut request = self.base_request.clone();
        if let Value::Object(ref mut map) = request {
            map.insert("params".to_string(), params);
        }
        TestEndpoint::new(path, Some(param_desc), Some(request))
    }
}

// Macro to create multiple endpoints easily
#[macro_export]
macro_rules! endpoints {
    ($framework:expr, $($path:expr),* $(,)?) => {{
        vec![
            $(
                $framework.create_endpoint($path),
            )*
        ]
    }};
}

// Macro to create endpoints with params
#[macro_export]
macro_rules! endpoints_with_params {
    ($framework:expr, $(($path:expr, $desc:expr, $params:expr)),* $(,)?) => {{
        vec![
            $(
                $framework.create_endpoint_with_params($path, $desc, $params),
            )*
        ]
    }};
}

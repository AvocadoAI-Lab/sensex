use super::common::{BASE_URL, get_test_client};
use super::test_utils::{TestEndpoint, test_endpoint, setup_test_directory};
use super::analyze_structure::{analyze_json_structure, print_json_structure};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::fs;
use std::path::Path;

const BACKEND_URL: &str = "http://127.0.0.1:3001";
const AGENT_ID: &str = "001";
const MODULE_NAME: &str = "agents";

#[tokio::test]
async fn test_agents_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    // 設置測試目錄
    setup_test_directory(MODULE_NAME)?;

    // 獲取認證 token
    let (_, token) = get_test_client().await;
    
    // 創建 HTTP client
    let client = Client::new();

    // 創建 headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", token))?);

    // 基本請求結構
    let base_request = serde_json::json!({
        "endpoint": BASE_URL,
        "token": token
    });

    // 定義所有要測試的endpoints
    let endpoints = vec![
        TestEndpoint::new("/agents", None, Some(base_request.clone())),
        TestEndpoint::new(
            &format!("/agents/{}/config/agent/global", AGENT_ID),
            Some("agent_id, component, configuration"),
            Some(serde_json::json!({
                "endpoint": BASE_URL,
                "token": token,
                "params": {
                    "agent_id": AGENT_ID,
                    "component": "agent",
                    "configuration": "global"
                }
            }))
        ),
        TestEndpoint::new(
            &format!("/agents/{}/group/is_sync", AGENT_ID),
            Some("agent_id"),
            Some(serde_json::json!({
                "endpoint": BASE_URL,
                "token": token,
                "params": {
                    "agent_id": AGENT_ID
                }
            }))
        ),
        TestEndpoint::new(
            &format!("/agents/{}/daemons/stats", AGENT_ID),
            Some("agent_id"),
            Some(serde_json::json!({
                "endpoint": BASE_URL,
                "token": token,
                "params": {
                    "agent_id": AGENT_ID
                }
            }))
        ),
        TestEndpoint::new(
            &format!("/agents/{}/stats/analysisd", AGENT_ID),
            Some("agent_id, component"),
            Some(serde_json::json!({
                "endpoint": BASE_URL,
                "token": token,
                "params": {
                    "agent_id": AGENT_ID,
                    "component": "analysisd"
                }
            }))
        ),
        TestEndpoint::new("/agents/no_group", None, Some(base_request.clone())),
        TestEndpoint::new("/agents/stats/distinct", None, Some(base_request.clone())),
        TestEndpoint::new("/agents/summary/os", None, Some(base_request.clone())),
        TestEndpoint::new("/agents/summary/status", None, Some(base_request.clone())),
    ];

    // 測試所有endpoints並分析其結構
    for endpoint in endpoints {
        match test_endpoint(&client, &headers, endpoint.clone(), BACKEND_URL, MODULE_NAME).await {
            Ok(json_value) => {
                // 分析響應結構
                let paths = analyze_json_structure(&json_value);
                let structure_output = print_json_structure(&paths, 0);
                
                // 創建結構分析文件
                let structure_file_path = Path::new("test_results")
                    .join(MODULE_NAME)
                    .join(format!("{}_structure.md", endpoint.path.replace("/", "_")));
                
                if let Some(parent) = structure_file_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                
                fs::write(&structure_file_path, format!(
                    "# Endpoint: {}\n\n## Response Structure:\n```\n{}\n```",
                    endpoint.path,
                    structure_output
                ))?;
            }
            Err(e) => {
                println!("Warning: Endpoint {} failed with error: {}", endpoint.path, e);
                continue;
            }
        }
    }

    println!("\n測試結果和結構分析已保存到 test_results/{} 目錄", MODULE_NAME);
    Ok(())
}

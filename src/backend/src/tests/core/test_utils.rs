use reqwest::Client;
use reqwest::header::HeaderMap;
use serde_json::Value;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use crate::tests::analyze_structure::{analyze_json_structure, print_json_structure};

#[derive(Clone)]
pub struct TestEndpoint {
    pub path: String,
    pub method: String,
    pub params: Option<String>,
    pub request_body: Option<Value>,
}

impl TestEndpoint {
    pub fn new(path: &str, params: Option<&str>, request_body: Option<Value>) -> Self {
        Self {
            path: path.to_string(),
            method: "GET".to_string(),
            params: params.map(String::from),
            request_body,
        }
    }
}

pub async fn write_test_report(module_name: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let report_dir = Path::new("test_results").join(module_name);
    if !report_dir.exists() {
        fs::create_dir_all(&report_dir)?;
    }

    let report_path = report_dir.join("test_report.md");
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    
    let report_content = format!(
        "# {} Test Report\n\n\
         Generated at: {}\n\n\
         {}\n",
        module_name, timestamp, content
    );

    fs::write(report_path, report_content)?;
    Ok(())
}

pub async fn test_endpoint(
    client: &Client,
    headers: &HeaderMap,
    endpoint: TestEndpoint,
    base_url: &str,
    module_name: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    println!("\nTesting {} {}", endpoint.method, endpoint.path);
    
    let response = client
        .post(&format!("{}{}", base_url, endpoint.path))
        .headers(headers.clone())
        .json(&endpoint.request_body.clone().unwrap_or(serde_json::json!({})))
        .send()
        .await?;

    let status = response.status().as_u16();
    let text = response.text().await?;
    
    // Parse response text to JSON Value
    let json_value: Value = serde_json::from_str(&text)?;
    
    // Write test result and structure analysis
    if let Err(e) = write_test_results(&endpoint, status, &json_value, module_name).await {
        println!("Warning: Failed to write test results: {}", e);
    }
    
    Ok(json_value)
}

async fn write_test_results(
    endpoint: &TestEndpoint,
    status: u16,
    json_value: &Value,
    module_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let report_dir = Path::new("test_results").join(module_name);
    if !report_dir.exists() {
        fs::create_dir_all(&report_dir)?;
    }

    // 將endpoint轉換為檔案名
    let file_name = endpoint.path
        .replace('/', "_")
        .replace(':', "_")
        .trim_start_matches('_')
        .to_string();
    
    // 寫入API響應結果
    let report_path = report_dir.join(format!("{}.md", file_name));
    let pretty_json = serde_json::to_string_pretty(&json_value)?;
    let result_text = format!("# {} {}\n\n\
                              ## Status Code\n{}\n\n\
                              ## Parameters\n{}\n\n\
                              ## Response\n```json\n{}\n```\n",
        endpoint.method,
        endpoint.path,
        status,
        endpoint.params.as_deref().unwrap_or("無"),
        pretty_json
    );
    fs::write(&report_path, result_text)?;

    // 寫入結構分析結果
    let structure_path = report_dir.join(format!("{}_structure.md", file_name));
    let paths = analyze_json_structure(&json_value);
    let structure_output = print_json_structure(&paths, 0);
    fs::write(&structure_path, format!(
        "# Endpoint: {}\n\n## Response Structure:\n```\n{}\n```",
        endpoint.path,
        structure_output
    ))?;

    // 更新索引文件
    let index_path = report_dir.join("README.md");
    if !index_path.exists() {
        fs::write(&index_path, &format!("# {} Endpoints 測試結果\n\n", module_name))?;
    }

    let mut index_file = OpenOptions::new()
        .append(true)
        .open(index_path)?;

    let index_entry = format!("- [{}](./{}.md)\n", endpoint.path, file_name);
    index_file.write_all(index_entry.as_bytes())?;

    Ok(())
}

pub fn setup_test_directory(module_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let report_dir = Path::new("test_results").join(module_name);
    if report_dir.exists() {
        fs::remove_dir_all(&report_dir)?;
    }
    fs::create_dir_all(&report_dir)?;
    Ok(())
}

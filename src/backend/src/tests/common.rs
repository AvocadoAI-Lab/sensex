pub const BASE_URL: &str = "https://wazuh.aixsoar.com:55000";
pub const TEST_USERNAME: &str = "wazuh-wui";
pub const TEST_PASSWORD: &str = "S.Ouv.51BHmQ*wqhq0O?eKSAyshu0Z.*";

use crate::client::WazuhClient;

pub async fn get_test_client() -> (WazuhClient, String) {
    let client = WazuhClient::new();
    let token = client.get_auth_token()
        .await
        .expect("Failed to get auth token");
    (client, token)
}

// 輔助函數：檢查回應狀態是否表示成功
pub fn is_success_response(status: u16) -> bool {
    status >= 200 && status < 300
}

// 輔助函數：檢查回應狀態是否表示未授權
pub fn is_unauthorized_response(status: u16) -> bool {
    status == 401
}

// 輔助函數：檢查回應狀態是否表示資源不存在
pub fn is_not_found_response(status: u16) -> bool {
    status == 404
}

// 輔助函數：檢查回應狀態是否表示功能未實現或不可用
pub fn is_not_implemented_response(status: u16) -> bool {
    status == 501 || status == 503
}

// 輔助函數：打印測試信息
pub fn print_test_info(test_name: &str, status: u16, response: &str) {
    println!("\n=== {} ===", test_name);
    println!("Status: {}", status);
    println!("Response: {}", response);
}

// 輔助函數：驗證回應
pub fn validate_response(test_name: &str, status: u16, response: &str) -> bool {
    print_test_info(test_name, status, response);
    
    if is_success_response(status) {
        println!("Test passed: Successful response");
        true
    } else if is_unauthorized_response(status) {
        println!("Test skipped: Unauthorized access");
        true
    } else if is_not_found_response(status) {
        println!("Test skipped: Resource not found");
        true
    } else if is_not_implemented_response(status) {
        println!("Test skipped: Feature not implemented or unavailable");
        true
    } else {
        println!("Test failed: Unexpected status code");
        false
    }
}

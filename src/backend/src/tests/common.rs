use crate::client::WazuhClient;

pub const BASE_URL: &str = "https://wazuh.aixsoar.com:55000";
pub const TEST_USERNAME: &str = "wazuh-wui";
pub const TEST_PASSWORD: &str = "S.Ouv.51BHmQ*wqhq0O?eKSAyshu0Z.*";

// 輔助函數：獲取測試用的 token
pub async fn get_test_token() -> String {
    let client = WazuhClient::new();
    client.get_auth_token().await.expect("Failed to get auth token")
}

// 輔助函數：建立已認證的客戶端
pub async fn get_test_client() -> (WazuhClient, String) {
    let client = WazuhClient::new();
    let token = get_test_token().await;
    (client, token)
}

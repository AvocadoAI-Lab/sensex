// Global test configuration
pub const WAZUH_URL: &str = "https://wazuh.aixsoar.com:55000";
pub const PROXY_URL: &str = "http://127.0.0.1:3001";
pub const TEST_USERNAME: &str = "wazuh-wui";
pub const TEST_PASSWORD: &str = "S.Ouv.51BHmQ*wqhq0O?eKSAyshu0Z.*";

// Commonly used test IDs
pub const TEST_AGENT_ID: &str = "005";
pub const TEST_GROUP_ID: &str = "default";

use crate::client::WazuhClient;

pub async fn get_test_client() -> (WazuhClient, String) {
    let client = WazuhClient::new();
    let token = client.get_auth_token()
        .await
        .expect("Failed to get auth token");
    (client, token)
}

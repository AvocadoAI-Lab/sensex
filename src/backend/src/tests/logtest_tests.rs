use super::common::{BASE_URL, get_test_client, validate_response};
use serde_json::json;

#[tokio::test]
async fn test_logtest_start() {
    let (client, token) = get_test_client().await;

    println!("Starting logtest session");
    let start_url = format!("{}/logtest", BASE_URL);
    
    let test_log = json!({
        "event": "Jun 24 11:54:23 hostname sshd[12345]: Accepted password for user from 192.168.1.1 port 54321",
        "log_format": "syslog",
        "location": "/var/log/syslog"
    });

    let response = client.put(&start_url, Some(&token), Some(test_log))  // 使用 PUT 而不是 POST
        .await
        .expect("Should start logtest session");
    
    let status = response.status();
    let response_text = response.text().await.expect("Should get response text");
    
    assert!(validate_response(
        "Logtest Start Test",
        status.as_u16(),
        &response_text
    ));
}

#[tokio::test]
async fn test_logtest_with_rules() {
    let (client, token) = get_test_client().await;

    println!("Testing logtest with custom rules");
    let test_url = format!("{}/logtest/run", BASE_URL);  // 使用正確的端點
    
    let test_config = json!({
        "event": "Jun 24 11:54:23 hostname sshd[12345]: Failed password for invalid user test from 192.168.1.1 port 54321",
        "log_format": "syslog",
        "location": "/var/log/auth.log"
    });

    let response = client.put(&test_url, Some(&token), Some(test_config))  // 使用 PUT 而不是 POST
        .await
        .expect("Should test log with rules");
    
    let status = response.status();
    let response_text = response.text().await.expect("Should get response text");
    
    assert!(validate_response(
        "Logtest With Rules Test",
        status.as_u16(),
        &response_text
    ));
}

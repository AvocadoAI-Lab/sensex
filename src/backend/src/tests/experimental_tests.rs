use super::common::{BASE_URL, get_test_client, validate_response};
use serde_json::json;

#[tokio::test]
async fn test_experimental_ciscat() {
    let (client, token) = get_test_client().await;

    println!("Testing experimental CIS-CAT scan");
    let ciscat_url = format!("{}/experimental/ciscat/results", BASE_URL);
    let response = client.get(&ciscat_url, Some(&token))
        .await
        .expect("Should get experimental CIS-CAT results");
    
    let status = response.status();
    let response_text = response.text().await.expect("Should get response text");
    
    assert!(validate_response(
        "Experimental CIS-CAT Test",
        status.as_u16(),
        &response_text
    ));
}

#[tokio::test]
async fn test_experimental_rules() {
    let (client, token) = get_test_client().await;

    println!("Testing experimental rules");
    let rules_url = format!("{}/experimental/rules", BASE_URL);
    
    let test_rule = json!({
        "rule": {
            "level": 5,
            "description": "Test experimental rule",
            "id": "100001",
            "pattern": "test pattern"
        }
    });

    let response = client.post(&rules_url, Some(&token), Some(test_rule))
        .await
        .expect("Should test experimental rule");
    
    let status = response.status();
    let response_text = response.text().await.expect("Should get response text");
    
    assert!(validate_response(
        "Experimental Rules Test",
        status.as_u16(),
        &response_text
    ));
}

#[tokio::test]
async fn test_experimental_decoders() {
    let (client, token) = get_test_client().await;

    println!("Testing experimental decoders");
    let decoders_url = format!("{}/experimental/decoders", BASE_URL);
    
    let test_decoder = json!({
        "decoder": {
            "name": "test_decoder",
            "program_name": "test_program",
            "pattern": "test pattern"
        }
    });

    let response = client.post(&decoders_url, Some(&token), Some(test_decoder))
        .await
        .expect("Should test experimental decoder");
    
    let status = response.status();
    let response_text = response.text().await.expect("Should get response text");
    
    assert!(validate_response(
        "Experimental Decoders Test",
        status.as_u16(),
        &response_text
    ));
}

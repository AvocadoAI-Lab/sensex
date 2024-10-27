use super::common::{BASE_URL, get_test_client};
use serde_json::Value;

#[tokio::test]
async fn test_security_users() {
    let (client, token) = get_test_client().await;

    println!("Getting security users");
    let users_url = format!("{}/security/users", BASE_URL);
    let response = client.get(&users_url, Some(&token))
        .await
        .expect("Should get users list");
    
    let status = response.status();
    println!("Security users status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get users list successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw users response: {}", response_text);
    
    let users_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("Users List Response: {:#?}", users_json);
}

#[tokio::test]
async fn test_security_roles() {
    let (client, token) = get_test_client().await;

    println!("Getting security roles");
    let roles_url = format!("{}/security/roles", BASE_URL);
    let response = client.get(&roles_url, Some(&token))
        .await
        .expect("Should get roles list");
    
    let status = response.status();
    println!("Security roles status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get roles list successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw roles response: {}", response_text);
    
    let roles_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("Roles List Response: {:#?}", roles_json);
}

#[tokio::test]
async fn test_security_policies() {
    let (client, token) = get_test_client().await;

    println!("Getting security policies");
    let policies_url = format!("{}/security/policies", BASE_URL);
    let response = client.get(&policies_url, Some(&token))
        .await
        .expect("Should get policies list");
    
    let status = response.status();
    println!("Security policies status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get policies list successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw policies response: {}", response_text);
    
    let policies_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("Policies List Response: {:#?}", policies_json);
}

#[tokio::test]
async fn test_security_rules() {
    let (client, token) = get_test_client().await;

    println!("Getting security rules");
    let rules_url = format!("{}/security/rules", BASE_URL);
    let response = client.get(&rules_url, Some(&token))
        .await
        .expect("Should get rules list");
    
    let status = response.status();
    println!("Security rules status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get rules list successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw rules response: {}", response_text);
    
    let rules_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("Security Rules Response: {:#?}", rules_json);
}

#[tokio::test]
async fn test_security_resources() {
    let (client, token) = get_test_client().await;

    println!("Getting security resources");
    let resources_url = format!("{}/security/resources", BASE_URL);
    let response = client.get(&resources_url, Some(&token))
        .await
        .expect("Should get resources list");
    
    let status = response.status();
    println!("Security resources status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get resources list successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw resources response: {}", response_text);
    
    let resources_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("Security Resources Response: {:#?}", resources_json);
}

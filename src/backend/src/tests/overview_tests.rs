use super::common::{BASE_URL, get_test_client, validate_response};

#[tokio::test]
async fn test_overview_agents() {
    let (client, token) = get_test_client().await;

    println!("Getting agents overview");
    let agents_url = format!("{}/overview/agents", BASE_URL);
    let response = client.get(&agents_url, Some(&token))
        .await
        .expect("Should get agents overview");
    
    let status = response.status();
    let response_text = response.text().await.expect("Should get response text");
    
    assert!(validate_response(
        "Agents Overview Test",
        status.as_u16(),
        &response_text
    ));
}

#[tokio::test]
async fn test_overview_security() {
    let (client, token) = get_test_client().await;

    println!("Getting security overview");
    let security_url = format!("{}/overview/security", BASE_URL);
    let response = client.get(&security_url, Some(&token))
        .await
        .expect("Should get security overview");
    
    let status = response.status();
    let response_text = response.text().await.expect("Should get response text");
    
    assert!(validate_response(
        "Security Overview Test",
        status.as_u16(),
        &response_text
    ));
}

#[tokio::test]
async fn test_overview_fim() {
    let (client, token) = get_test_client().await;

    println!("Getting FIM overview");
    let fim_url = format!("{}/overview/fim", BASE_URL);
    let response = client.get(&fim_url, Some(&token))
        .await
        .expect("Should get FIM overview");
    
    let status = response.status();
    let response_text = response.text().await.expect("Should get response text");
    
    assert!(validate_response(
        "FIM Overview Test",
        status.as_u16(),
        &response_text
    ));
}

#[tokio::test]
async fn test_overview_vulnerabilities() {
    let (client, token) = get_test_client().await;

    println!("Getting vulnerabilities overview");
    let vuln_url = format!("{}/overview/vulnerabilities", BASE_URL);
    let response = client.get(&vuln_url, Some(&token))
        .await
        .expect("Should get vulnerabilities overview");
    
    let status = response.status();
    let response_text = response.text().await.expect("Should get response text");
    
    assert!(validate_response(
        "Vulnerabilities Overview Test",
        status.as_u16(),
        &response_text
    ));
}

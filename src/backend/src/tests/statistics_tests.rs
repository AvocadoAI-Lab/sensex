use super::common::{BASE_URL, get_test_client, validate_response};

#[tokio::test]
async fn test_statistics_hourly() {
    let (client, token) = get_test_client().await;

    println!("Getting hourly statistics");
    let hourly_url = format!("{}/statistics/hourly", BASE_URL);
    let response = client.get(&hourly_url, Some(&token))
        .await
        .expect("Should get hourly statistics");
    
    let status = response.status();
    let response_text = response.text().await.expect("Should get response text");
    
    assert!(validate_response(
        "Hourly Statistics Test",
        status.as_u16(),
        &response_text
    ));
}

#[tokio::test]
async fn test_statistics_weekly() {
    let (client, token) = get_test_client().await;

    println!("Getting weekly statistics");
    let weekly_url = format!("{}/statistics/weekly", BASE_URL);
    let response = client.get(&weekly_url, Some(&token))
        .await
        .expect("Should get weekly statistics");
    
    let status = response.status();
    let response_text = response.text().await.expect("Should get response text");
    
    assert!(validate_response(
        "Weekly Statistics Test",
        status.as_u16(),
        &response_text
    ));
}

#[tokio::test]
async fn test_statistics_analysisd() {
    let (client, token) = get_test_client().await;

    println!("Getting analysisd statistics");
    let analysisd_url = format!("{}/statistics/analysisd", BASE_URL);
    let response = client.get(&analysisd_url, Some(&token))
        .await
        .expect("Should get analysisd statistics");
    
    let status = response.status();
    let response_text = response.text().await.expect("Should get response text");
    
    assert!(validate_response(
        "Analysisd Statistics Test",
        status.as_u16(),
        &response_text
    ));
}

#[tokio::test]
async fn test_statistics_remoted() {
    let (client, token) = get_test_client().await;

    println!("Getting remoted statistics");
    let remoted_url = format!("{}/statistics/remoted", BASE_URL);
    let response = client.get(&remoted_url, Some(&token))
        .await
        .expect("Should get remoted statistics");
    
    let status = response.status();
    let response_text = response.text().await.expect("Should get response text");
    
    assert!(validate_response(
        "Remoted Statistics Test",
        status.as_u16(),
        &response_text
    ));
}

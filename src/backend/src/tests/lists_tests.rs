use super::test_framework::TestFramework;
use crate::endpoints;

const MODULE_NAME: &str = "lists";

#[tokio::test]
async fn test_lists_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    let endpoints = endpoints!(framework,
        "/lists",
        "/lists/files"
    );

    framework.test_endpoints(endpoints).await?;
    Ok(())
}

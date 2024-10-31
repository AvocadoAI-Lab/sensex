use super::test_framework::TestFramework;
use crate::endpoints;

const MODULE_NAME: &str = "security";

#[tokio::test]
async fn test_security_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    let endpoints = endpoints!(framework,
        "/security/actions",
        "/security/resources",
        "/security/config"
    );

    framework.test_endpoints(endpoints).await?;
    Ok(())
}

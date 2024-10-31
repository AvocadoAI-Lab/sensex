use super::test_framework::TestFramework;
use crate::endpoints;

const MODULE_NAME: &str = "rules";

#[tokio::test]
async fn test_rules_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    let endpoints = endpoints!(framework,
        "/rules",
        "/rules/groups",
        "/rules/files"
    );

    framework.test_endpoints(endpoints).await?;
    Ok(())
}

use super::test_framework::TestFramework;
use crate::endpoints;

const MODULE_NAME: &str = "tasks";

#[tokio::test]
async fn test_tasks_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    let endpoints = endpoints!(framework,
        "/tasks/status"
    );

    framework.test_endpoints(endpoints).await?;
    Ok(())
}

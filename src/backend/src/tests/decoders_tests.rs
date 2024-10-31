use super::test_framework::TestFramework;
use crate::endpoints;

const MODULE_NAME: &str = "decoders";

#[tokio::test]
async fn test_decoders_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    let endpoints = endpoints!(framework,
        "/decoders",
        "/decoders/files",
        "/decoders/parents"
    );

    framework.test_endpoints(endpoints).await?;
    Ok(())
}

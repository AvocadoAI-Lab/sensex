use crate::tests::core::TestFramework;

const MODULE_NAME: &str = "groups_list";

#[tokio::test]
async fn test_groups_list() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    // Create endpoint for getting groups
    let groups_endpoint = framework.create_endpoint("/groups");
    
    // Get groups response
    let groups_response = framework.test_endpoint(groups_endpoint).await?;

    // Extract and store group names
    let mut group_names = Vec::new();
    if let Some(affected_items) = groups_response["data"]["affected_items"].as_array() {
        for group in affected_items {
            if let Some(group_name) = group["name"].as_str() {
                group_names.push(group_name.to_string());
            }
        }
    }

    // Print group names for verification
    println!("\nFound groups:");
    for name in &group_names {
        println!("- {}", name);
    }

    Ok(())
}

use serde_json::json;

#[tokio::test]
async fn test_contract_is_operational() -> Result<(), Box<dyn std::error::Error>> {
    let contract_wasm = near_workspaces::compile_project("./").await?;

    test_basics_on(&contract_wasm).await?;
    Ok(())
}

async fn test_basics_on(contract_wasm: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    
    // Deploy the contract
    let contract = sandbox.dev_deploy(contract_wasm).await?;

    // Create two accounts: weatherman and regular user
    let weatherman_account = sandbox.dev_create_account().await?;
    let regular_user = sandbox.dev_create_account().await?;

    // Initialize the contract with weatherman account
    let init_outcome = contract
        .call("new")
        .args_json(json!({
            "weatherman": weatherman_account.id()
        }))
        .transact()
        .await?;
    assert!(init_outcome.is_success());

    // Test 1: Weatherman can set temperature
    let set_temp_outcome = weatherman_account
        .call(contract.id(), "set_temperature")
        .args_json(json!({"temperature": 25.5}))
        .transact()
        .await?;
    assert!(set_temp_outcome.is_success());

    // Verify temperature was set correctly
    let temp_outcome = contract.view("get_temperature").args_json(json!({})).await?;
    assert_eq!(temp_outcome.json::<f64>()?, 25.5);

    // Test 2: Regular user cannot set temperature
    let unauthorized_outcome = regular_user
        .call(contract.id(), "set_temperature")
        .args_json(json!({"temperature": 30.0}))
        .transact()
        .await;
    
    // This should fail with an error about unauthorized access
    assert!(unauthorized_outcome.is_err() || 
           unauthorized_outcome.unwrap().is_failure());

    // Test 3: Verify temperature hasn't changed after failed attempt
    let temp_outcome = contract.view("get_temperature").args_json(json!({})).await?;
    assert_eq!(temp_outcome.json::<f64>()?, 25.5);

    // Test 4: Verify weatherman address
    let weatherman_outcome = contract.view("get_weatherman").args_json(json!({})).await?;
    assert_eq!(weatherman_outcome.json::<String>()?, weatherman_account.id().to_string());

    Ok(())
}
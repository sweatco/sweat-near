use std::fs;

use anyhow::Result;
use near_self_update::UpdateApiIntegration;

use crate::prepare::{prepare_contract, IntegrationContext};

#[tokio::test]
async fn update() -> Result<()> {
    let mut context = prepare_contract().await?;

    assert_eq!(context.ft_contract().contract_version().await?, "sweat-1.2.1");

    context
        .ft_contract()
        .update_contract(vec![], None)
        .expect_error("Unauthorized access! Only oracle can call that!")
        .await?;

    let big_version = fs::read("../res_test/sweat_big_version.wasm")?;
    let small_version = fs::read("../res/sweat.wasm")?;

    let oracle = context.oracle().await?;

    context
        .ft_contract()
        .update_contract(small_version, "test_update_callback".to_string().into())
        .with_user(&oracle)
        .expect_log("test_update_callback called")
        .await?;

    assert_eq!(context.ft_contract().contract_version().await?, "sweat-1.2.1");

    context
        .ft_contract()
        .update_contract(big_version.clone(), "aaaaaa".to_string().into())
        .with_user(&oracle)
        .expect_error("MethodResolveError(MethodNotFound)")
        .await?;

    context
        .ft_contract()
        .update_contract(big_version.clone(), None)
        .with_user(&oracle)
        .dont_expect_log("test_update_callback called")
        .await?;

    assert_eq!(context.ft_contract().contract_version().await?, "sweat-9999.9.9");

    Ok(())
}

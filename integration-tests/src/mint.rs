use integration_utils::{integration_contract::IntegrationContract, misc::ToNear};
use near_sdk::json_types::{U128, U64};
use sweat_model::{FungibleTokenCoreIntegration, SweatApiIntegration};

use crate::prepare::{prepare_contract, IntegrationContext};

const TARGET_BALANCE: u128 = 9999999976902174720;
const TARGET_STEPS_SINCE_TGE: u32 = 10_000;

#[tokio::test]
async fn test_mint() -> anyhow::Result<()> {
    let mut context = prepare_contract().await?;
    let user = context.alice().await?;
    let oracle = context.oracle().await?;

    let result = context.ft_contract().get_steps_since_tge().await?;
    assert_eq!(result, U64(0));

    let result = context.ft_contract().formula(U64(0), TARGET_STEPS_SINCE_TGE).await?;
    assert_eq!(result, U128(TARGET_BALANCE));

    context
        .ft_contract()
        .with_user(&oracle)
        .record_batch(vec![(user.to_near(), 10_000u32)])
        .await?;

    let result = context.ft_contract().ft_balance_of(oracle.to_near()).await?;
    assert_eq!(result, U128(TARGET_BALANCE * 5 / 100));

    let result = context.ft_contract().ft_balance_of(user.to_near()).await?;
    assert_eq!(result, U128(TARGET_BALANCE * 95 / 100));

    let result = context.ft_contract().get_steps_since_tge().await?;
    assert_eq!(result, U64(TARGET_STEPS_SINCE_TGE as u64));

    Ok(())
}

#![cfg(test)]

use integration_utils::misc::ToNear;
use near_sdk::json_types::U64;
use sweat_model::{FungibleTokenCoreIntegration, SweatApiIntegration, SweatDeferIntegration};

use crate::prepare::{prepare_contract, IntegrationContext};

#[tokio::test]
async fn test_defer() -> anyhow::Result<()> {
    const BATCH_SIZE: u32 = 135;
    const CLAIM_AMOUNT: u32 = 10_000;

    let mut context = prepare_contract().await?;
    let oracle = context.oracle().await?;
    let alice = context.alice().await?;
    let claim_contract_account = context.claim_contract().as_account().to_near();

    let holder_balance = context
        .ft_contract()
        .ft_balance_of(claim_contract_account.clone())
        .call()
        .await?;

    assert_eq!(holder_balance.0, 0);

    let mut total_fee = 0;
    let mut total_for_user = 0;

    for i in 0..BATCH_SIZE {
        let steps_since_tge = i * CLAIM_AMOUNT;
        let steps = CLAIM_AMOUNT;

        let minted = context
            .ft_contract()
            .formula(U64(steps_since_tge.into()), steps)
            .call()
            .await?
            .0;

        let fee = (minted * 5).div_ceil(100);
        let for_user = minted - fee;

        total_fee += fee;
        total_for_user += for_user;
    }

    let batch: Vec<_> = (0..BATCH_SIZE).map(|_| (alice.to_near(), CLAIM_AMOUNT)).collect();

    context
        .ft_contract()
        .defer_batch(batch, claim_contract_account.clone())
        .with_user(&oracle)
        .call()
        .await?;

    let alice_balance = context.ft_contract().ft_balance_of(alice.to_near()).call().await?;
    assert_eq!(0, alice_balance.0);

    let claim_contract_balance = context
        .ft_contract()
        .ft_balance_of(claim_contract_account.clone())
        .call()
        .await?;

    let oracle_balance = context.ft_contract().ft_balance_of(oracle.to_near()).call().await?;

    assert_eq!(oracle_balance.0, total_fee);
    assert_eq!(claim_contract_balance.0, total_for_user);

    Ok(())
}

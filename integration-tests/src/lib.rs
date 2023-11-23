#![cfg(test)]

use integration_utils::{integration_contract::IntegrationContract, misc::ToNear};
use sweat_model::{FungibleTokenCoreIntegration, SweatApiIntegration, SweatDeferIntegration};

use crate::prepare::{prepare_contract, IntegrationContext};

mod formula;
mod interface;
mod mint;
mod prepare;
mod transfer;

#[tokio::test]
async fn happy_flow() -> anyhow::Result<()> {
    let mut context = prepare_contract().await?;

    let alice = context.alice().await?;
    let manager = context.manager().await?;

    assert_eq!(
        99999995378125008,
        context.ft_contract().formula(100_000.into(), 100).await?.0
    );

    context
        .ft_contract()
        .tge_mint(&alice.to_near(), 100_000_000.into())
        .await?;

    assert_eq!(
        100_000_000,
        context.ft_contract().ft_balance_of(alice.to_near()).await?.0
    );

    context
        .ft_contract()
        .with_user(&manager)
        .defer_batch(vec![(alice.to_near(), 1000)], manager.to_near())
        .await?;

    Ok(())
}

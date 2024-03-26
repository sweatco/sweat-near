#![cfg(test)]

use anyhow::Result;
use near_workspaces::types::Gas;
use nitka::measure::utils::pretty_gas_string;
use sweat_model::SweatApiIntegration;

use crate::{prepare::IntegrationContext, prepare_contract};

#[ignore]
#[tokio::test]
async fn single_record_batch() -> Result<()> {
    let gas = measure_record_batch().await?;

    println!("{}", pretty_gas_string(gas));

    Ok(())
}

async fn measure_record_batch() -> Result<Gas> {
    let mut context = prepare_contract().await?;

    let oracle = context.oracle().await?;

    let gas = context
        .ft_contract()
        .record_batch(Default::default())
        .with_user(&oracle)
        .result()
        .await?
        .total_gas_burnt;

    Ok(gas)
}

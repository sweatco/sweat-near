#![cfg(test)]

use anyhow::Result;
use integration_utils::measure::outcome_storage::OutcomeStorage;
use near_workspaces::types::Gas;
use sweat_model::SweatApiIntegration;

use crate::{prepare::IntegrationContext, prepare_contract};

#[ignore]
#[tokio::test]
async fn measure_record_batch_test() -> Result<()> {
    // async fn batch_penalty() -> Result<()> {
    //     let measured = scoped_command_measure(
    //         generate_permutations(
    //             &[RegisterProductCommand::Flexible6Months6Percents],
    //             &measure_jars_range(),
    //         ),
    //         measure_batch_penalty,
    //     )
    //     .await?;
    //
    //     let mut map: HashMap<RegisterProductCommand, Vec<(Gas, usize)>> = HashMap::new();
    //
    //     for measure in measured {
    //         map.entry(measure.0 .0).or_default().push((measure.1, measure.0 .1));
    //     }
    //
    //     let map: HashMap<RegisterProductCommand, _> = map
    //         .into_iter()
    //         .map(|(key, gas_cost)| {
    //             let mut differences: Vec<i128> = Vec::new();
    //             for i in 1..gas_cost.len() {
    //                 let diff = gas_cost[i].0.as_gas() as i128 - gas_cost[i - 1].0.as_gas() as i128;
    //                 differences.push(diff);
    //             }
    //
    //             (key, MeasureData::new(gas_cost, differences))
    //         })
    //         .collect();
    //
    //     append_measure("batch_penalty", map)
    // }
    //
    // retry_until_ok(batch_penalty).await?;

    Ok(())
}

#[ignore]
#[tokio::test]
async fn single_record_batch() -> anyhow::Result<()> {
    let gas = measure_record_batch().await?;

    dbg!(&gas);

    Ok(())
}

async fn measure_record_batch() -> Result<Gas> {
    let mut context = prepare_contract().await?;

    let oracle = context.oracle().await?;

    let (gas, _) = OutcomeStorage::measure_total(
        &oracle,
        context
            .ft_contract()
            .record_batch(Default::default())
            .with_user(&oracle)
            .call(),
    )
    .await?;

    Ok(gas)
}

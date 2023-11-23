#![cfg(test)]

use near_sdk::json_types::{U128, U64};
use serde_json::json;

use crate::prepare::{prepare_contract, IntegrationContext};

const SWEAT_WASM_FILEPATH: &str = "./res/sweat.wasm";
const TARGET_BALANCE: u128 = 0;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut context = prepare_contract().await?;

    // can't use sandbox on M1 because of
    // https://github.com/near/workspaces-rs/issues/110
    // ☹️
    let worker = workspaces::testnet().await?;
    let wasm = std::fs::read(SWEAT_WASM_FILEPATH)?;
    let contract = worker.dev_deploy(&wasm).await?;
    let oracle = worker.dev_create_account().await?;
    let user = worker.dev_create_account().await?;

    let result = contract.call("new").args_json(json!({})).transact().await?;
    println!("deploy: {:#?}", result);

    let result = contract
        .view("get_steps_since_tge")
        .args_json(json!({}))
        .await?
        .json::<U64>()?;
    assert_eq!(result, U64(0));

    let result = contract
        .view("formula")
        .args_json(json!({
            "steps_since_tge": U64(0),
            "steps" : 10_000u32,
        }))
        .await?
        .json::<U128>()?;
    assert_eq!(result, U128(9999999991287398400));

    let result = contract
        .as_account()
        .call(contract.id(), "add_oracle")
        .args_json(json!({
            "account_id": oracle.id(),
        }))
        .transact()
        .await?;
    println!("add_oracle: {:#?}", result);

    let result = oracle
        .call(contract.id(), "record_batch")
        .args_json(json!({
            "steps_batch": vec![(user.id(), 10_000u32)],
        }))
        .transact()
        .await?;

    println!("record_batch: {:#?}", result);

    let result = contract
        .view("ft_balance_of")
        .args_json(json!({
            "account_id": oracle.id(),
        }))
        .await?
        .json::<U128>()?;
    assert_eq!(result, U128(9999999991287398400 * 5 / 100));

    let result = contract
        .view("ft_balance_of")
        .args_json(json!({
            "account_id": user.id(),
        }))
        .await?
        .json::<U128>()?;
    assert_eq!(result, U128(9999999991287398400 * 95 / 100));

    let result = contract
        .view("get_steps_since_tge")
        .args_json(json!({}))
        .await?
        .json::<U64>()?;
    assert_eq!(result, U64(10_000));

    Ok(())
}

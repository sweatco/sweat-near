use serde_json::json;

use near_sdk::json_types::{U128, U64};
use workspaces::prelude::*;
use workspaces::{AccountId, Contract, DevNetwork, Worker};

const SWEAT_WASM_FILEPATH: &str = "./target/wasm32-unknown-unknown/release/sweat.wasm";

async fn assert_steps_from_tge(
    assert_to: U64,
    contract: &Contract,
    worker: &Worker<impl DevNetwork>,
) -> anyhow::Result<()> {
    let result = worker
        .view(contract.id(), "get_steps_from_tge", Vec::new())
        .await?
        .json::<U64>()?;
    assert_eq!(result.0, assert_to.0);
    Ok(())
}

async fn assert_formula(
    steps_from_tge: U64,
    steps: u16,
    assert_to: U128,
    contract: &Contract,
    worker: &Worker<impl DevNetwork>,
) -> anyhow::Result<()> {
    let result = contract
        .view(
            &worker,
            "formula",
            json!({
                "steps_from_tge": steps_from_tge,
                "steps" : steps,
            })
            .to_string()
            .into_bytes(),
        )
        .await?
        .json::<U128>()?;
    assert_eq!(result.0, assert_to.0);
    Ok(())
}

async fn assert_balance(
    user: &AccountId,
    assert_to: U128,
    contract: &Contract,
    worker: &Worker<impl DevNetwork>,
) -> anyhow::Result<()> {
    let result = contract
        .view(
            &worker,
            "ft_balance_of",
            json!({
                "account_id": user,
            })
            .to_string()
            .into_bytes(),
        )
        .await?
        .json::<U128>()?;
    assert_eq!(result.0, assert_to.0);
    Ok(())
}

async fn assert_steps_from_purgatory(
    user: &AccountId,
    assert_to: u16,
    contract: &Contract,
    worker: &Worker<impl DevNetwork>,
) -> anyhow::Result<()> {
    let result = contract
        .view(
            &worker,
            "get_steps_from_purgatory",
            json!({
                "account_id": user,
            })
            .to_string()
            .into_bytes(),
        )
        .await?
        .json::<u16>()?;
    assert_eq!(result, assert_to);
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 🙁 can't use sandbox on M1 because of https://github.com/near/workspaces-rs/issues/110
    // let worker = workspaces::sandbox().await?;
    let worker = workspaces::testnet().await?;
    let wasm = std::fs::read(SWEAT_WASM_FILEPATH)?;
    let contract = worker.dev_deploy(&wasm).await?;
    let oracle = worker.dev_create_account().await?;
    let user = worker.dev_create_account().await?;

    // deploy
    let _result = contract.call(&worker, "new").transact().await;

    assert_steps_from_tge(U64(0), &contract, &worker).await?;
    assert_formula(U64(0), 10_000u16, U128(9999999991287398400), &contract, &worker).await?;
    assert_balance(user.id(), U128(0), &contract, &worker).await?;
    assert_balance(oracle.id(), U128(0), &contract, &worker).await?;

    let result = oracle
        .call(&worker, contract.id(), "attest_steps")
        .args_json(json!({
            "steps_batch": vec![(user.id(), 10_000u16)],
        }))?
        .transact()
        .await;
    // only oracle can attest steps
    println!(": {:#?}", result);
    assert!(result.is_err());

    let result = oracle
        .call(&worker, contract.id(), "add_oracle")
        .args_json(json!({
            "account_id": oracle.id(),
        }))?
        .transact()
        .await;
    // only token account can add oracles
    println!(": {:#?}", result);
    assert!(result.is_err());

    let result = contract
        .call(&worker, "add_oracle")
        .args_json(json!({
            "account_id": oracle.id(),
        }))?
        .transact()
        .await;
    assert!(result.is_ok());

    let result = oracle
        .call(&worker, contract.id(), "attest_steps")
        .args_json(json!({
            "steps_batch": vec![(user.id(), 10_000u16)],
        }))?
        .transact()
        .await;
    // oracle is not registered
    println!(": {:#?}", result);
    assert!(result.is_err());
    assert_balance(user.id(), U128(0), &contract, &worker).await?;
    assert_balance(oracle.id(), U128(0), &contract, &worker).await?;
    assert_balance(contract.id(), U128(0), &contract, &worker).await?;
    assert_steps_from_purgatory(user.id(), 0, &contract, &worker).await?;
    assert_steps_from_tge(U64(0), &contract, &worker).await?;

    let result = contract
        .call(&worker, "storage_deposit")
        .args_json((oracle.id(), Option::<bool>::None))?
        .max_gas()
        .deposit(near_sdk::env::storage_byte_cost() * 125)
        .transact()
        .await?;
    assert!(result.is_success());

    let result = oracle
        .call(&worker, contract.id(), "attest_steps")
        .args_json(json!({
            "steps_batch": vec![(user.id(), 10_000u16)],
        }))?
        .transact()
        .await;
    // will skip unregistered users
    println!(": {:#?}", result);
    assert!(result.is_ok());
    assert_balance(user.id(), U128(0), &contract, &worker).await?;
    assert_balance(oracle.id(), U128(0), &contract, &worker).await?;
    assert_balance(contract.id(), U128(0), &contract, &worker).await?;
    assert_steps_from_purgatory(user.id(), 0, &contract, &worker).await?;
    assert_steps_from_tge(U64(0), &contract, &worker).await?;

    let result = oracle
        .call(&worker, contract.id(), "storage_deposit")
        .args_json((user.id(), Option::<bool>::None))?
        .max_gas()
        .deposit(near_sdk::env::storage_byte_cost() * 125)
        .transact()
        .await?;
    assert!(result.is_success());

    let result = oracle
        .call(&worker, contract.id(), "attest_steps")
        .args_json(json!({
            "steps_batch": vec![(user.id(), 10_000u16)],
        }))?
        .transact()
        .await;
    assert!(result.is_ok());
    assert_balance(user.id(), U128(0), &contract, &worker).await?;
    assert_balance(oracle.id(), U128(9999999991287398400 * 5 / 100), &contract, &worker).await?;
    assert_balance(contract.id(), U128(0), &contract, &worker).await?;
    assert_steps_from_purgatory(user.id(), 10_000u16, &contract, &worker).await?;
    assert_steps_from_tge(U64(0), &contract, &worker).await?;

    Ok(())
}

use near_sdk::json_types::{U128, U64};
use serde_json::json;

const SWEAT_WASM_FILEPATH: &str = "./target/wasm32-unknown-unknown/release/sweat.wasm";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // can't use sandbox on M1 because of
    // https://github.com/near/workspaces-rs/issues/110
    // ☹️
    let worker = workspaces::testnet().await?;
    let wasm = std::fs::read(SWEAT_WASM_FILEPATH)?;
    let contract = worker.dev_deploy(&wasm).await?;
    let oracle = worker.dev_create_account().await?;
    let user = worker.dev_create_account().await?;

    let result = contract
        .as_account()
        .call(contract.id(), "new")
        .args_json(json!({}))
        .transact()
        .await?;

    println!("deploy: {:#?}", result);

    let result = contract.view("get_steps_since_tge").await?.json::<U64>()?;
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

    let new_account = worker.dev_create_account().await?;
    // This will fail because storage is not registered for this new account
    let res = oracle
        .call(contract.id(), "ft_transfer")
        .args_json((new_account.id(), U128(9499999991723028480), Option::<bool>::None))
        .max_gas()
        .deposit(1)
        .transact()
        .await;
    assert!(res.is_err());

    let res = contract
        .as_account()
        .call(contract.id(), "storage_deposit")
        .args_json((new_account.id(), Option::<bool>::None))
        .max_gas()
        .deposit(near_sdk::env::storage_byte_cost() * 125)
        .transact()
        .await?;
    assert!(res.is_success());

    // Transfer all tokens from intmainreturn to new account
    let res = oracle
        .call(contract.id(), "ft_transfer")
        .args_json((
            new_account.id(),
            U128(9999999991287398400 * 5 / 100),
            Option::<bool>::None,
        ))
        .max_gas()
        .deposit(1)
        .transact()
        .await?;
    assert!(res.is_success());
    println!("ft_transfer: {:#?}", res);

    Ok(())
}

use near_sdk::json_types::{U128, U64};
use serde_json::json;
use workspaces::prelude::*;

const SWEAT_WASM_FILEPATH: &str = "./res/sweat.wasm";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // can't use sandbox on M1 because of
    // https://github.com/near/workspaces-rs/issues/110
    // ☹️
    let worker = workspaces::testnet().await?;
    let wasm = std::fs::read(SWEAT_WASM_FILEPATH)?;
    let contract = worker.dev_deploy(&wasm).await?;
    let oracle = worker.dev_create_account().await?;
    let holding_account = worker.dev_create_account().await?;
    let user = worker.dev_create_account().await?;

    println!("🏃Run deploy");
    let result = contract.call("new").args_json(json!({})).transact().await?;
    println!("✅ Deploy: {:#?}", result);

    let result = worker.view(contract.id(), "get_steps_since_tge").await?.json::<U64>()?;
    assert_eq!(result, U64(0));

    println!("🏃Run formula");
    let result = contract
        .view("formula")
        .args_json(json!({
            "steps_since_tge": U64(0),
            "steps" : 10_000u32,
        }))
        .await?
        .json::<U128>()?;
    assert_eq!(result, U128(9999999976902174720));

    println!("🏃Run add_oracle");
    let result = contract
        .call("add_oracle")
        .args_json(json!({
            "account_id": oracle.id(),
        }))
        .transact()
        .await?;
    println!("✅ Add oracle: {:#?}", result);

    println!("🏃Run defer_batch");
    let result = oracle
        .call(contract.id(), "defer_batch")
        .args_json(json!({
            "steps_batch": vec![(user.id(), 10_000u32)],
            "holding_account_id": holding_account.id(),
        }))
        .gas(300 * 1_000_000_000_000)
        .transact()
        .await?;
    println!("✅ Defer batch: {:#?}", result);

    println!("🏃Request user balance");
    let user_balance = contract
        .view("ft_balance_of")
        .args_json(json!({
            "account_id": user.id(),
        }))
        .await?
        .json::<U128>()?;
    assert_eq!(user_balance, U128(0));
    println!("✅ Checked user balance");

    println!("🏃Request holder balance");
    let holder_balance = contract
        .view("ft_balance_of")
        .args_json(json!({
            "account_id": holding_account.id(),
        }))
        .await?
        .json::<U128>()?;
    assert_eq!(holder_balance, U128(9999999976902174720 * 95 / 100));
    println!("✅ Checked holder balance");

    println!("🏃Request oracle balance");
    let oracle_balance = contract
        .view("ft_balance_of")
        .args_json(json!({
            "account_id": oracle.id(),
        }))
        .await?
        .json::<U128>()?;
    assert_eq!(oracle_balance, U128(9999999976902174720 * 5 / 100));
    println!("✅ Checked oracle balance");

    Ok(())
}

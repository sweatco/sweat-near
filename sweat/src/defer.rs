use near_sdk::{serde_json::json, Gas, Promise};
use sweat_model::SweatDefer;

use crate::*;

#[near_bindgen]
impl SweatDefer for Contract {
    fn defer_batch(&mut self, steps_batch: Vec<(AccountId, u16)>, holding_account_id: AccountId) -> PromiseOrValue<()> {
        require!(
            self.oracles.contains(&env::predecessor_account_id()),
            "Unauthorized access! Only oracle can call that!"
        );

        let mut accounts_tokens: Vec<(AccountId, U128)> = Vec::new();
        let mut total_to_mint: u128 = 0;
        let mut total_fee: u128 = 0;

        for (account_id, step_count) in steps_batch {
            let (amount, fee) = self.calculate_tokens_amount(step_count);
            self.steps_since_tge.0 += step_count as u64;

            accounts_tokens.push((account_id, U128(amount)));
            total_to_mint += amount;
            total_fee += fee;
        }

        internal_deposit(&mut self.token, &env::predecessor_account_id(), total_fee);
        internal_deposit(&mut self.token, &holding_account_id, total_to_mint);

        let hold_arguments = json!({
            "amounts": accounts_tokens,
        });

        Promise::new(holding_account_id.clone())
            .function_call(
                "record_batch_for_hold".to_string(),
                hold_arguments.to_string().into_bytes(),
                0,
                Gas(20 * 1_000_000_000_000),
            )
            .then(
                ext_ft_transfer_callback::ext(env::current_account_id())
                    .with_static_gas(Gas(5 * 1_000_000_000_000))
                    .on_transfer(holding_account_id, U128(total_to_mint)),
            )
            .into()
    }
}

#[ext_contract(ext_ft_transfer_callback)]
pub trait FungibleTokenTransferCallback {
    fn on_transfer(&mut self, receiver_id: AccountId, amount: U128);
}

impl FungibleTokenTransferCallback for Contract {
    fn on_transfer(&mut self, receiver_id: AccountId, amount: U128) {
        if !is_promise_success() {
            rollback_internal_deposit(&mut self.token, &receiver_id, amount.0);
        }
    }
}

use integration_trait::make_integration_version;
use near_sdk::{AccountId, PromiseOrValue};
use near_sdk::json_types::{U128, U64};

#[make_integration_version]
pub trait SweatApi {
    fn new(postfix: Option<String>) -> Self;
    fn add_oracle(&mut self, account_id: &AccountId);
    fn remove_oracle(&mut self, account_id: &AccountId);
    fn get_oracles(&self) -> Vec<AccountId>;
    fn tge_mint(&mut self, account_id: &AccountId, amount: U128);
    fn tge_mint_batch(&mut self, batch: Vec<(AccountId, U128)>);
    fn burn(&mut self, amount: &U128);
    fn get_steps_since_tge(&self) -> U64;
    fn record_batch(&mut self, steps_batch: Vec<(AccountId, u16)>);
    fn formula(&self, steps_since_tge: U64, steps: u16) -> U128;
}

/// Copy of near_sdk trait to use in integration tests
#[make_integration_version]
pub trait FungibleTokenCore {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128>;
    fn ft_total_supply(&self) -> U128;
    fn ft_balance_of(&self, account_id: AccountId) -> U128;
}
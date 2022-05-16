use near_contract_standards::fungible_token::events::FtBurn;
use near_contract_standards::fungible_token::events::FtMint;
use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata, FungibleTokenMetadataProvider};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LookupSet};
use near_sdk::json_types::{U128, U64};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue};

mod math;

pub const DAY_IN_NANOS: u64 = 86_400_000_000_000;
pub const TWO_DAYS_IN_NANOS: u64 = 2 * 86_400_000_000_000;
pub const DAILY_STEP_CONVERSION_LIMIT: u16 = 10_000;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    oracles: LookupSet<AccountId>,
    token: FungibleToken,
    steps_from_tge: U64,
    purgatory: LookupMap<AccountId, (u16, u64)>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            oracles: LookupSet::new(b"s"),
            token: FungibleToken::new(b"t"),
            steps_from_tge: U64::from(0),
            purgatory: LookupMap::new(b"l"),
        }
    }

    #[private]
    pub fn mint_tge(&mut self, amount: U128, account_for: AccountId) {
        assert_eq!(env::predecessor_account_id(), env::current_account_id());
        self.token.internal_register_account(&account_for);
        self.token.internal_deposit(&env::predecessor_account_id(), amount.0);
    }

    #[private]
    pub fn add_oracle(&mut self, account_id: &AccountId) {
        assert_eq!(env::predecessor_account_id(), env::current_account_id());
        self.oracles.insert(account_id);
    }

    #[private]
    pub fn remove_oracle(&mut self, account_id: &AccountId) {
        assert_eq!(env::predecessor_account_id(), env::current_account_id());
        self.oracles.remove(account_id);
    }

    pub fn get_steps_from_tge(&self) -> U64 {
        self.steps_from_tge
    }

    pub fn get_steps_from_purgatory(&self, account_id: &AccountId) -> u16 {
        let (steps, _) = self.purgatory.get(account_id).unwrap_or((0, 0));
        steps
    }

    pub fn burn(&mut self, amount: &U128) {
        self.token.internal_withdraw(&env::predecessor_account_id(), amount.0);
        FtBurn {
            owner_id: &env::predecessor_account_id(),
            amount: amount,
            memo: None,
        }
        .emit()
    }

    pub fn attest_steps(&mut self, steps_batch: Vec<(AccountId, u16)>) {
        assert!(self.oracles.contains(&env::predecessor_account_id()));
        let mut oracle_fee: u128 = 0;
        for (account_id, steps) in steps_batch.into_iter() {
            if !self.token.accounts.contains_key(&account_id) {
                // don't mint for unregistered accounts
                continue;
            }
            let time: u64 = env::block_timestamp();
            let (prev_steps, prev_time) = self.purgatory.get(&account_id).unwrap_or((0, 0));
            let time_diff = time - prev_time;
            if prev_time == 0 {
                // first walkchain
                self.purgatory.insert(&account_id, &(steps, time));
            } else if time_diff <= DAY_IN_NANOS {
                // accumulate steps for a day in purgatory
                self.purgatory.insert(&account_id, &(prev_steps + steps, prev_time));
            } else if DAY_IN_NANOS < time_diff && time_diff <= TWO_DAYS_IN_NANOS {
                // mint yesterday steps from purgatory
                let steps_to_mint = u16::min(2 * DAILY_STEP_CONVERSION_LIMIT, prev_steps);
                let sweat_to_mint: u128 = self.formula(self.steps_from_tge, steps_to_mint).0;
                let to_oracle: u128 = sweat_to_mint * 5 / 100;
                let to_user: u128 = sweat_to_mint - to_oracle;
                oracle_fee = oracle_fee + to_oracle;
                mint(&mut self.token, &account_id, to_user);
                self.steps_from_tge.0 += steps_to_mint as u64;
                // add new walkchain
                self.purgatory.insert(&account_id, &(steps, time));
            } else {
                // if TWO_DAYS_IN_NANOS < time_diff
                // inactivity fee - erase old steps
                self.purgatory.insert(&account_id, &(steps, time));
            }
        }
        mint(&mut self.token, &env::predecessor_account_id(), oracle_fee);
    }

    pub fn formula(&self, steps_from_tge: U64, steps: u16) -> U128 {
        U128(math::formula(steps_from_tge.0 as f64, steps as f64))
    }
}

fn mint(token: &mut FungibleToken, account_id: &AccountId, amount: Balance) {
    token.internal_deposit(account_id, amount);
    FtMint {
        owner_id: account_id,
        amount: &U128(amount),
        memo: None,
    }
    .emit()
}

near_contract_standards::impl_fungible_token_core!(Contract, token);
near_contract_standards::impl_fungible_token_storage!(Contract, token);

pub const ICON: &str = "data:image/svg+xml,%3Csvg viewBox='0 0 100 100' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Crect width='100' height='100' rx='50' fill='%23FF0D75'/%3E%3Cg clip-path='url(%23clip0_283_2788)'%3E%3Cpath d='M39.4653 77.5455L19.0089 40.02L35.5411 22.2805L55.9975 59.806L39.4653 77.5455Z' stroke='white' stroke-width='10'/%3E%3Cpath d='M66.0253 77.8531L45.569 40.3276L62.1012 22.5882L82.5576 60.1136L66.0253 77.8531Z' stroke='white' stroke-width='10'/%3E%3C/g%3E%3Cdefs%3E%3CclipPath id='clip0_283_2788'%3E%3Crect width='100' height='56' fill='white' transform='translate(0 22)'/%3E%3C/clipPath%3E%3C/defs%3E%3C/svg%3E%0A";

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        FungibleTokenMetadata {
            spec: "ft-1.0".to_string(),
            name: "SWEAT".to_string(),
            symbol: "SWEAT".to_string(),
            icon: Some(String::from(ICON)),
            reference: None,
            reference_hash: None,
            decimals: 18,
        }
    }
}

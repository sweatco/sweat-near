use std::str::FromStr;

use nitka::near_sdk::AccountId;
use sweat_model::SweatContract;

pub(crate) trait ContractAccount {
    fn account(&self) -> AccountId;
}

impl ContractAccount for SweatContract<'_> {
    fn account(&self) -> AccountId {
        AccountId::from_str(&self.contract.as_account().id().to_string()).unwrap()
    }
}

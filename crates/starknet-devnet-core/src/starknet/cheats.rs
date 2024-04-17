use std::collections::HashSet;

use starknet_types::contract_address::ContractAddress;


#[derive(Default, Clone)]
pub(crate) struct Cheats {
    impersonated_accounts: HashSet<ContractAddress>,
    auto_impersonate: bool,
}

impl Cheats {
    pub(crate) fn impersonate_account(&mut self, account: ContractAddress) {
        self.impersonated_accounts.insert(account);
    }
    pub(crate) fn stop_impersonating_account(&mut self, account: &ContractAddress) {
        self.impersonated_accounts.remove(account);
    }
    pub(crate) fn is_impersonated(&self, account: &ContractAddress) -> bool {
        self.auto_impersonate || self.impersonated_accounts.contains(account)
    }
    // pub(crate) fn auto_impersonate(&mut self) {
    //     self.auto_impersonate = true;
    // }
    // pub(crate) fn stop_auto_impersonate(&mut self) {
    //     self.auto_impersonate = false;
    // }
}

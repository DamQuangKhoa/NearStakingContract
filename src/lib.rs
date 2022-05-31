use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, BlockHeight, BorshStorageKey, EpochHeight,
    PanicOnDefault, Promise,
};
use near_sdk::json_types::U128;

use crate::account::*;
use crate::config::*;
use crate::enumeration::*;
use crate::internal::*;
use crate::utils::*;

mod account;
mod config;
mod enumeration;
mod internal;
mod utils;

#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    AccountKey,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[near_bindgen]
pub struct StakingContract {
    pub owner_id: AccountId,
    pub ft_contract_id: AccountId,
    pub config: Config,
    pub total_stake_balance: Balance,
    pub total_paid_reward_balance: Balance,
    pub total_stakers: Balance,
    pub pre_reward: Balance,
    pub last_block_balance_change: BlockHeight,
    pub accounts: LookupMap<AccountId, Account>,
    pub paused: bool,
    pub pause_at_block: BlockHeight,
}
#[near_bindgen]
impl StakingContract {
    #[init]
    pub fn new_default_config(owner_id: AccountId, ft_contract_id: AccountId) -> Self {
        Self::new(owner_id, ft_contract_id, Config::default())
    }

    #[init]
    pub fn new(owner_id: AccountId, ft_contract_id: AccountId, config: Config) -> Self {
        StakingContract {
            owner_id,
            ft_contract_id,
            config,
            total_stake_balance: 0,
            total_paid_reward_balance: 0,
            total_stakers: 0,
            pre_reward: 0,
            last_block_balance_change: env::block_index(),
            paused: false,
            accounts: LookupMap::new(StorageKey::AccountKey),
            pause_at_block: 0,
        }
    }

    #[payable]
    pub fn storage_deposite(&mut self, account_id: Option<AccountId>) {
        assert_at_least_one_yocto();
        let account = account_id.unwrap_or_else(|| env::predecessor_account_id());
        let account_stake = self.accounts.get(&account);

        if account_stake.is_some() {
            // refund toan bo token deposit voi account da dky
            refund_deposit(0);
        } else {
            // Tao Account Moi
            let before_storage_usage = env::storage_usage();

            let after_storage_usage = env::storage_usage();

            refund_deposit(after_storage_usage - before_storage_usage);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, MockedBlockchain};

    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(accounts(0))
            .predecessor_account_id(accounts(0))
            .is_view(is_view);
        builder
    }

    #[test]
    fn test_init_contract() {
        let context = get_context(false);
        testing_env!(context.build());

        let config: Config = Config {
            reward_numerator: 500,
            reward_demurator: 100000,
        };

        let contract: StakingContract =
            StakingContract::new(accounts(1).to_string(), "ft_contract".to_string(), config);

        assert_eq!(contract.owner_id, accounts(1).to_string());
        assert_eq!(contract.ft_contract_id, "ft_contract".to_string());
        assert_eq!(config.reward_numerator, contract.config.reward_numerator);
        assert_eq!(contract.paused, false);
    }
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

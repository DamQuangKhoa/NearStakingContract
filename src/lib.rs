use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, BlockHeight, BorshStorageKey, EpochHeight,
    PanicOnDefault, Promise,
    PromiseOrValue
};

use crate::config::*;
use crate::account::*;
use crate::enumeration::*;
use crate::internal::*;
use crate::utils::*;
use crate::core_impl::*;

mod account;
mod config;
mod enumeration;
mod internal;
mod utils;
mod core_impl;

#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    AccountKey,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct StakingContractV1 {
    pub owner_id: AccountId,
    pub ft_contract_id: AccountId,
    pub config: Config,
    pub total_stake_balance: Balance,
    pub total_paid_reward_balance: Balance,
    pub total_stakers: Balance,
    pub pre_reward: Balance,
    pub last_block_balance_change: BlockHeight,
    pub accounts: LookupMap<AccountId, UpgradableAccount>,
    pub paused: bool,
    pub pause_at_block: BlockHeight,
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
    pub accounts: LookupMap<AccountId, UpgradableAccount>,
    pub paused: bool,
    pub pause_at_block: BlockHeight,
    pub new_data: U128,
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
            new_data: U128(0),
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
            self.internal_register_account(account.clone());

            let after_storage_usage = env::storage_usage();

            refund_deposit(after_storage_usage - before_storage_usage);
        }
    }

    pub fn get_new_data(&self) -> U128 {
        self.new_data
    }

    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let contract_v1: StakingContractV1 = env::state_read().expect("Can not read state data");

        StakingContract {
            owner_id: contract_v1.owner_id,
            ft_contract_id: contract_v1.ft_contract_id,
            config: contract_v1.config,
            total_stake_balance: contract_v1.total_stake_balance,
            total_paid_reward_balance: contract_v1.total_paid_reward_balance,
            total_stakers: contract_v1.total_paid_reward_balance,
            pre_reward: contract_v1.pre_reward,
            last_block_balance_change: contract_v1.last_block_balance_change,
            paused: contract_v1.paused,
            accounts: contract_v1.accounts,
            pause_at_block: contract_v1.pause_at_block,
            new_data: U128(10),
        }
    }

    pub fn is_pause(&self) -> bool {
        self.paused
    }

    pub fn get_ft_contract(&self) -> String {
        self.ft_contract_id.to_string()
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

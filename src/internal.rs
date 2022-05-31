use near_sdk::json_types::U128;

use crate::*;

#[near_bindgen]
impl StakingContract {
    pub(crate) fn internal_register_account(&mut self , account_id: AccountId) {
        let account = Account {
            stake_balance: 0,
            pre_reward: 0,
            last_block_balance_change: env::block_index(),
            unstake_balance: 0,
            unstake_start_timestamp: 0,
            unstake_available_epoch: 0,
        };

        self.accounts.insert(&account_id, &account);
    }

    pub fn storage_balance_of(&self, account_id: AccountId) -> U128 {
        let account = self.accounts.get(&account_id);

        if account.is_some(){
            U128(1)
        } else {
            U128(0)
        }
    }

    pub fn is_pause(&self) -> bool {
        self.paused
    }

    pub fn internal_calculate_account_reward(&self, account: &Account) -> Balance {
        let lasted_block = if self.paused {
            self.pause_at_block 
        } else {
            env::block_index()
        };

        let diff_lock = lasted_block - account.last_block_balance_change;
        let reward = (account.stake_balance * self.config.reward_numerator as u128 * diff_lock as u128) / self.config.reward_demurator as u128;
        
        reward
    }

    pub fn internal_calculate_pool_reward(&self) -> Balance {
        let lasted_block = if self.paused {
            self.pause_at_block 
        } else {
            env::block_index()
        };

        let diff_lock = lasted_block - self.last_block_balance_change;
        let reward = (self.total_stake_balance * self.config.reward_numerator as u128 * diff_lock as u128) / self.config.reward_demurator as u128;
        
        reward
    }
}

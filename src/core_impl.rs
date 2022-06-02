
use crate::*;

pub trait FungibleTokenReceiver {
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128>;
}

impl FungibleTokenReceiver for StakingContract {
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128> {
        self.internal_deposit_and_stake(sender_id, amount.0);
        PromiseOrValue::Value(U128(0))
    }
}

#[near_bindgen]
impl StakingContract {
    
}
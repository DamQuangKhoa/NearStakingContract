use crate::*;

pub(crate) fn assert_at_least_one_yocto() {
    assert!(
        env::attached_deposit() >= 1,
        "Required Attachedd Deposit Of At Least 1 yoctoNear"
    )
}

pub(crate) fn assert_one_yocto() {
    assert_eq!(
        env::attached_deposit(),
        1,
        "Required Attachedd Deposit Of At Least 1 yoctoNear"
    )
}

pub(crate) fn refund_deposit(storage_used: u64) {
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    let attach_deposit = env::attached_deposit();

    assert!(attach_deposit >= required_cost, "Must attach {} yoctoNear to cover storage", required_cost);
    let refund = attach_deposit - required_cost;

    if refund > 0 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}
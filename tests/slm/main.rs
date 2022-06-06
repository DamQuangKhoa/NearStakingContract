use near_sdk::{json_types::U128, serde_json::json};
use near_sdk_sim::{init_simulator, to_yocto, UserAccount, DEFAULT_GAS, STORAGE_AMOUNT};
use staking_contract::AccountJson;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    FT_CONTRACT_WASM_FILE => "token-test/line-ft.wasm",
    STAKING_CONTRACT_WASM_FILE => "out/staking-contract.wasm"
}

const FT_CONTRACT_ID: &str = "ft_contract";
const FT_TOTAL_SUPPLY: &str = "1000000000000000";
const STAKING_CONTRACT_ID: &str = "staking_contract";
const STAKING_FT_AMOUNT: &str = "500000000000000";
const ALICE_DEPOSIT_AMOUNT: &str = "100000000000000";

pub fn init() -> (UserAccount, UserAccount, UserAccount, UserAccount) {
    let root: UserAccount = init_simulator(None);
    let alice: UserAccount = root.create_user("alice".to_string(), to_yocto("100"));

    // Deploy and init FT Token

    let ft_contract = root.deploy_and_init(
        &FT_CONTRACT_WASM_FILE,
        FT_CONTRACT_ID.to_string(),
        "new_default_meta",
        &json!({
            "owner_id": alice.account_id(),
            "total_supply": FT_TOTAL_SUPPLY
        })
        .to_string()
        .as_bytes(),
        STORAGE_AMOUNT,
        DEFAULT_GAS,
    );

    // Deploy and init Staking Contract

    let staking_contract: UserAccount = root.deploy_and_init(
        &STAKING_CONTRACT_WASM_FILE,
        STAKING_CONTRACT_ID.to_string(),
        "new_default_config",
        &json!({
            "owner_id": alice.account_id(),
            "ft_contract_id": FT_CONTRACT_ID
        })
        .to_string()
        .as_bytes(),
        STORAGE_AMOUNT,
        DEFAULT_GAS,
    );

    // Storage deposite ft contract

    root.call(
        ft_contract.account_id(),
        "storage_deposit",
        &json!({
            "account_id": staking_contract.account_id()
        })
        .to_string()
        .as_bytes(),
        DEFAULT_GAS,
        to_yocto("0.01"),
    );

    alice.call(
        ft_contract.account_id(),
        "ft_transfer",
        &json!({
            "receiver_id": staking_contract.account_id(),
            "amount": STAKING_FT_AMOUNT
        })
        .to_string()
        .as_bytes(),
        DEFAULT_GAS,
        1,
    );

    (root, alice, ft_contract, staking_contract)
}

#[test]
pub fn test_deposit_and_stake() {
    let (root, alice, ft_contract, staking_contract) = init();

    // Storage Deposit
    alice.call(
        staking_contract.account_id(),
        "storage_deposit",
        &json!({}).to_string().as_bytes(),
        DEFAULT_GAS,
        to_yocto("0.01"),
    );

    // Deposit Token
    alice.call(
        ft_contract.account_id(),
        "ft_transfer_call",
        &json!({
            "receiver_id": staking_contract.account_id(),
            "amount": ALICE_DEPOSIT_AMOUNT,
            "msg": ""
        })
        .to_string()
        .as_bytes(),
        DEFAULT_GAS,
        1,
    );

    let account_json: AccountJson = root
        .view(
            staking_contract.account_id(),
            "get_account_info",
            &json!({
                "receiver_id": staking_contract.account_id(),
                "amount": ALICE_DEPOSIT_AMOUNT,
                "msg": ""
            })
            .to_string()
            .as_bytes(),
        )
        .unwrap_json();

    assert_eq!(account_json.account_id, alice.account_id());
    assert_eq!(
        account_json.stake_balance,
        U128(10000000000000000000000000000)
    );
}

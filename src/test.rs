use crate::{NativeFlowContract, NativeFlowContractClient};
use soroban_sdk::{testutils::*, Env};

fn create_test_token(env: &Env, admin: &soroban_sdk::Address) -> soroban_sdk::Address {
    env.register_stellar_asset_contract(admin.clone())
}

#[test]
fn test_subscribe_creates_subscription() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, NativeFlowContract);
    let client = NativeFlowContractClient::new(&env, &contract_id);

    let user = soroban_sdk::Address::generate(&env);
    let merchant = soroban_sdk::Address::generate(&env);
    let token = create_test_token(&env, &soroban_sdk::Address::generate(&env));

    let amount = 1000i128;
    let interval = 86400u64; // 1 day in seconds
    let keeper_bounty = 100i128;

    client.subscribe(&user, &merchant, &token, &amount, &interval, &keeper_bounty);

    let subscription = client.get_subscription(&user, &merchant);
    assert!(subscription.is_some());

    let config = subscription.unwrap();
    assert_eq!(config.amount, amount);
    assert_eq!(config.interval, interval);
    assert_eq!(config.keeper_bounty, keeper_bounty);
}

#[test]
fn test_cancel_removes_subscription() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, NativeFlowContract);
    let client = NativeFlowContractClient::new(&env, &contract_id);

    let user = soroban_sdk::Address::generate(&env);
    let merchant = soroban_sdk::Address::generate(&env);
    let token = create_test_token(&env, &soroban_sdk::Address::generate(&env));

    let amount = 1000i128;
    let interval = 86400u64;
    let keeper_bounty = 100i128;

    client.subscribe(&user, &merchant, &token, &amount, &interval, &keeper_bounty);

    let subscription = client.get_subscription(&user, &merchant);
    assert!(subscription.is_some());

    client.cancel(&user, &merchant);

    let subscription = client.get_subscription(&user, &merchant);
    assert!(subscription.is_none());
}

#[test]
fn test_execute_payment_rejected_before_interval() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().with_mut(|ledger| ledger.timestamp = 100000u64);

    let contract_id = env.register_contract(None, NativeFlowContract);
    let client = NativeFlowContractClient::new(&env, &contract_id);

    let user = soroban_sdk::Address::generate(&env);
    let merchant = soroban_sdk::Address::generate(&env);
    let keeper = soroban_sdk::Address::generate(&env);

    let admin = soroban_sdk::Address::generate(&env);
    let token = create_test_token(&env, &admin);

    let amount = 1000i128;
    let interval = 86400u64;
    let keeper_bounty = 100i128;

    client.subscribe(&user, &merchant, &token, &amount, &interval, &keeper_bounty);

    // Call execute_payment and expect false (since payment is not yet due)
    let result = client.execute_payment(&user, &merchant, &keeper);
    assert!(!result);
}

#[test]
fn test_execute_payment_succeeds_after_interval() {
    let env = Env::default();
    env.mock_all_auths();
    let initial_time = 100000u64;
    env.ledger()
        .with_mut(|ledger| ledger.timestamp = initial_time);

    let contract_id = env.register_contract(None, NativeFlowContract);
    let client = NativeFlowContractClient::new(&env, &contract_id);

    let user = soroban_sdk::Address::generate(&env);
    let merchant = soroban_sdk::Address::generate(&env);
    let keeper = soroban_sdk::Address::generate(&env);
    let admin = soroban_sdk::Address::generate(&env);

    let token = create_test_token(&env, &admin);

    // Mint some tokens to the user to avoid insufficient balance error
    let token_client = soroban_sdk::token::StellarAssetClient::new(&env, &token);
    token_client.mint(&user, &2000i128);

    let amount = 1000i128;
    let interval = 86400u64;
    let keeper_bounty = 100i128;

    // Approve the contract to spend user's tokens
    let token_client_regular = soroban_sdk::token::Client::new(&env, &token);
    token_client_regular.approve(&user, &contract_id, &(amount + keeper_bounty), &99999u32);

    client.subscribe(&user, &merchant, &token, &amount, &interval, &keeper_bounty);

    env.ledger()
        .with_mut(|ledger| ledger.timestamp = initial_time + interval + 1);

    let result = client.execute_payment(&user, &merchant, &keeper);
    assert!(result);

    let subscription = client.get_subscription(&user, &merchant);
    assert!(subscription.is_some());

    let config = subscription.unwrap();
    assert_eq!(config.last_charge, initial_time + interval + 1);
}

#[test]
fn test_cancel_as_merchant() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, NativeFlowContract);
    let client = NativeFlowContractClient::new(&env, &contract_id);

    let user = soroban_sdk::Address::generate(&env);
    let merchant = soroban_sdk::Address::generate(&env);
    let token = create_test_token(&env, &soroban_sdk::Address::generate(&env));

    let amount = 1000i128;
    let interval = 86400u64;
    let keeper_bounty = 100i128;

    client.subscribe(&user, &merchant, &token, &amount, &interval, &keeper_bounty);

    let subscription = client.get_subscription(&user, &merchant);
    assert!(subscription.is_some());

    client.cancel_merch(&user, &merchant);

    let subscription = client.get_subscription(&user, &merchant);
    assert!(subscription.is_none());
}

#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, token, Address, Env, Vec as SorobanVec,
};

#[derive(Clone)]
#[contracttype]
pub struct SubscriptionConfig {
    pub token: Address,
    pub amount: i128,
    pub interval: u64,
    pub last_charge: u64,
    pub keeper_bounty: i128,
}

#[contract]
pub struct NativeFlowContract;

#[contractimpl]
impl NativeFlowContract {
    /// Initialize a subscription between a user and merchant
    ///
    /// # Arguments
    /// * `user` - The user account address
    /// * `merchant` - The merchant account address
    /// * `token` - The token address for payments
    /// * `amount` - The payment amount per interval
    /// * `interval` - The interval in seconds between charges
    /// * `keeper_bounty` - The bounty for the keeper executing the payment
    pub fn subscribe(
        env: Env,
        user: Address,
        merchant: Address,
        token: Address,
        amount: i128,
        interval: u64,
        keeper_bounty: i128,
    ) {
        user.require_auth();

        let subscription_key = SorobanVec::from_array(&env, [user.clone(), merchant.clone()]);

        let current_ledger_timestamp = env.ledger().timestamp();

        let config = SubscriptionConfig {
            token,
            amount,
            interval,
            last_charge: current_ledger_timestamp,
            keeper_bounty,
        };

        env.storage().persistent().set(&subscription_key, &config);

        env.events()
            .publish((symbol_short!("subscribe"),), (&user, &merchant));
    }

    /// Cancel a subscription between a user and merchant
    ///
    /// # Arguments
    /// * `user` - The user account address
    /// * `merchant` - The merchant account address
    pub fn cancel(env: Env, user: Address, merchant: Address) {
        user.require_auth();

        let subscription_key = SorobanVec::from_array(&env, [user.clone(), merchant.clone()]);

        env.storage().persistent().remove(&subscription_key);

        env.events()
            .publish((symbol_short!("cancel"),), (&user, &merchant));
    }

    /// Cancel subscription as merchant
    ///
    /// # Arguments
    /// * `user` - The user account address
    /// * `merchant` - The merchant account address
    pub fn cancel_merch(env: Env, user: Address, merchant: Address) {
        merchant.require_auth();

        let subscription_key = SorobanVec::from_array(&env, [user.clone(), merchant.clone()]);

        env.storage().persistent().remove(&subscription_key);

        env.events()
            .publish((symbol_short!("cancel_m"),), (&user, &merchant));
    }

    /// Execute a payment for an active subscription
    ///
    /// # Arguments
    /// * `user` - The user account address
    /// * `merchant` - The merchant account address
    /// * `keeper` - The keeper account address (receives the bounty)
    pub fn execute_payment(env: Env, user: Address, merchant: Address, keeper: Address) -> bool {
        let subscription_key = SorobanVec::from_array(&env, [user.clone(), merchant.clone()]);

        let mut config: SubscriptionConfig = env
            .storage()
            .persistent()
            .get::<SorobanVec<Address>, SubscriptionConfig>(&subscription_key)
            .expect("Subscription does not exist");

        let current_ledger_timestamp = env.ledger().timestamp();

        require_payment_due(&config, current_ledger_timestamp);

        let token_client = token::Client::new(&env, &config.token);

        token_client.transfer_from(
            &user,
            &merchant,
            &env.current_contract_address(),
            &config.amount,
        );

        token_client.transfer_from(
            &user,
            &keeper,
            &env.current_contract_address(),
            &config.keeper_bounty,
        );

        config.last_charge = current_ledger_timestamp;

        env.storage().persistent().set(&subscription_key, &config);

        env.events().publish(
            (symbol_short!("payment"),),
            (&user, &merchant, current_ledger_timestamp),
        );

        true
    }

    /// Query subscription configuration
    ///
    /// # Arguments
    /// * `user` - The user account address
    /// * `merchant` - The merchant account address
    pub fn get_subscription(
        env: Env,
        user: Address,
        merchant: Address,
    ) -> Option<SubscriptionConfig> {
        let subscription_key = SorobanVec::from_array(&env, [user, merchant]);

        env.storage()
            .persistent()
            .get::<SorobanVec<Address>, SubscriptionConfig>(&subscription_key)
    }
}

/// Helper function to ensure payment is due
fn require_payment_due(config: &SubscriptionConfig, current_time: u64) {
    let payment_due_time = config.last_charge + config.interval;
    if current_time < payment_due_time {
        panic!("Payment is not yet due");
    }
}

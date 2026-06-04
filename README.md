# NativeFlow: Decentralized Recurring Payments Protocol

## Project Description

**NativeFlow** is a non-custodial, decentralized recurring payments protocol built on Stellar's Soroban smart contract platform. It enables automated pull-payments between users and merchants without requiring custodial wallets or centralized intermediaries.

The protocol leverages Soroban's robust execution environment to execute recurring payment instructions securely, with built-in authentication and state management on the Stellar ledger. Each subscription is uniquely identified by a (User, Merchant) tuple, storing payment configuration persistently on-chain.

### Key Characteristics

- **Non-Custodial**: Users retain full control over their assets; the contract executes transfers directly from user wallets
- **Decentralized**: All payment logic and state is managed on-chain via Soroban smart contracts
- **Native Token Support**: Compatible with any Stellar-issued token through the native Soroban Token Client
- **Automated Execution**: Keeper addresses can trigger payments on behalf of subscribed merchants when intervals are met
- **Transparent & Auditable**: All transactions are recorded on the immutable Stellar ledger

## Core Features

### 1. User Authentication
- All subscription creation and cancellation operations require explicit user authorization via `require_auth()`
- Users maintain full control over their subscription lifecycle

### 2. Automated Pull-Payments
- Merchants define recurring payment intervals (in ledger seconds)
- Designated keeper addresses execute payments automatically when intervals elapse
- Keepers receive bounties for transaction execution costs

### 3. Native Token Compatibility
- Subscriptions work with any Stellar-issued token (USDC, EURC, custom tokens)
- Payment amounts and keeper bounties are configurable per subscription
- Direct token transfers via Soroban's native Token Client

### 4. Flexible Cancellation
- Users can cancel their subscriptions at any time
- Merchants can also revoke subscriptions via `cancel_merch()`
- Cancellation immediately removes subscription state from the ledger

## Architecture Overview

### Smart Contract State Layout

Subscriptions are stored in persistent contract storage with a composite key structure:

```
Key: Vec<Address> = [user_address, merchant_address]
Value: SubscriptionConfig {
    token: Address,           // Token contract address
    amount: i128,             // Payment amount per interval
    interval: u64,            // Interval duration in ledger seconds
    last_charge: u64,         // Timestamp of last charge
    keeper_bounty: i128,      // Bounty paid to keeper for execution
}
```

### Execution Constraints

**Payment Authorization**:
- `execute_payment()` validates that the subscription exists
- Enforces time constraint: `current_ledger_timestamp >= last_charge + interval`
- Prevents out-of-schedule payments

**Token Transfers**:
- Primary transfer: `amount` tokens from user → merchant
- Secondary transfer: `keeper_bounty` tokens from user → keeper (transaction executor)
- Both transfers execute atomically; failure of either reverts the entire transaction

**State Updates**:
- `last_charge` is updated to the current ledger timestamp after successful payment
- Persistent storage is immediately synced to reflect the updated state

## Contract Functions

### `subscribe`
Initializes a new subscription between a user and merchant.

**Parameters:**
- `user`: Subscriber account address (must authorize)
- `merchant`: Recipient merchant address
- `token`: Token contract address for payments
- `amount`: Payment amount per interval (in token decimals)
- `interval`: Interval duration in ledger seconds
- `keeper_bounty`: Bounty paid to payment executor

**Authorization:** Required from `user`

---

### `cancel`
Cancels an active subscription (user-initiated).

**Parameters:**
- `user`: Subscriber address (must authorize)
- `merchant`: Associated merchant address

**Authorization:** Required from `user`

---

### `cancel_merch`
Cancels an active subscription (merchant-initiated).

**Parameters:**
- `user`: Subscriber address
- `merchant`: Merchant address (must authorize)

**Authorization:** Required from `merchant`

---

### `execute_payment`
Executes a recurring payment if the interval has elapsed.

**Parameters:**
- `user`: Subscriber address
- `merchant`: Recipient merchant address
- `keeper`: Payment executor address (receives bounty)

**Returns:** `bool` (true on success)

**Constraints:**
- Subscription must exist
- Current time ≥ last_charge + interval
- User must have sufficient token balance for payment + bounty

---

### `get_subscription`
Queries subscription configuration details.

**Parameters:**
- `user`: Subscriber address
- `merchant`: Merchant address

**Returns:** `Option<SubscriptionConfig>` (None if subscription does not exist)

## Deployed Contract Address

### Stellar Testnet Deployment

**Contract ID:** `CAX3DPOZCQCDEMDJNHDZ5ZRG4NJVLNOUKIBLDRXUSQQJ4MEEFKQGM3RS`

**Network:** Stellar Test Stellar Public Network ; September 2015  
**RPC Endpoint:** https://soroban-testnet.stellar.org:443

To interact with the deployed contract, use the Soroban CLI or the Stellar JavaScript SDK with the contract address above.

## How to Run Tests

The test suite validates core functionality including subscription creation, payment enforcement, and state updates.

### Prerequisites

- Rust 1.96+ with the `wasm32-unknown-unknown` target installed
- Soroban SDK 20.5+

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_subscribe_creates_subscription

# Run with output
cargo test -- --nocapture

# Run in release mode
cargo test --release
```

### Test Coverage

1. **Subscription Creation**: Verifies successful subscription initialization and state storage
2. **Payment Interval Validation**: Ensures `execute_payment` rejects early payments
3. **Successful Payment Execution**: Confirms proper token transfers after interval elapses
4. **State Updates**: Validates `last_charge` timestamp updates post-execution
5. **Merchant Cancellation**: Tests merchant-initiated subscription revocation

## Building the Contract

### Development Build

```bash
cargo build --target wasm32-unknown-unknown
```

### Optimized Release Build

```bash
cargo build --target wasm32-unknown-unknown --release
```

The optimized WASM binary is located at:
```
target/wasm32-unknown-unknown/release/nativeflow_contract.wasm
```

## Security Considerations

### Audited Aspects

- ✅ Code formatting verified with `cargo fmt`
- ✅ Rust best practices enforced with `cargo clippy`
- ✅ WASM binary size optimized for Soroban constraints

### Key Security Properties

1. **Authorization Enforcement**: All state-modifying operations require explicit authorization from authorized parties
2. **Atomic Transfers**: Payment execution ensures both transfers succeed or both fail
3. **Immutable Timestamps**: Ledger timestamps are cryptographically secured by the Stellar network
4. **No Arithmetic Overflow**: Soroban's i128 type prevents integer overflow attacks on amounts

### Recommendations

- **Token Vetting**: Verify token contract code before subscribing to ensure no malicious transfer hooks
- **Keeper Selection**: Only trust keepers with strong operational security practices
- **Amount Limits**: Set reasonable subscription amounts to limit exposure per execution

## Development & Deployment Workflow

### 1. Local Testing

```bash
cargo test
```

### 2. Format & Lint

```bash
cargo fmt --check
cargo clippy --lib --target wasm32-unknown-unknown -- -D warnings
```

### 3. Build Release WASM

```bash
cargo build --target wasm32-unknown-unknown --release
```

### 4. Deploy to Testnet

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/nativeflow_contract.wasm \
  --source-account <DEPLOYER_ACCOUNT> \
  --network testnet
```

### 5. Interact with Contract

```bash
# Subscribe to recurring payment
soroban contract invoke \
  --id CAX3DPOZCQCDEMDJNHDZ5ZRG4NJVLNOUKIBLDRXUSQQJ4MEEFKQGM3RS \
  --source-account <USER_ACCOUNT> \
  --network testnet \
  -- subscribe \
  --user <USER_ADDRESS> \
  --merchant <MERCHANT_ADDRESS> \
  --token <TOKEN_ADDRESS> \
  --amount 10000000 \
  --interval 86400 \
  --keeper-bounty 100000
```

## Repository Structure

```
nativeflow-contract/
├── src/
│   ├── lib.rs          # Main smart contract implementation
│   └── test.rs         # Unit tests (reference)
├── Cargo.toml          # Project manifest
├── .gitignore          # Git configuration
└── README.md           # This file
```

## License

This project is provided as-is for educational and demonstration purposes.

## Support & Feedback

For questions, feedback, or issues related to NativeFlow, please refer to the Soroban documentation:
- [Soroban Docs](https://developers.stellar.org/docs/smart-contracts)
- [Stellar Testnet Faucet](https://friendbot.stellar.org/)

---

**Built with ❤️ using Soroban SDK v20.5**
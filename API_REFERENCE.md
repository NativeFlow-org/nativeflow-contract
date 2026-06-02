# NativeFlow Contract API Reference

## Contract Overview
- **Name:** NativeFlow
- **Type:** Soroban Smart Contract
- **Language:** Rust
- **SDK Version:** 20.5
- **WASM Binary Size:** 5.4 KB
- **Network:** Stellar Testnet

---

## Data Structures

### SubscriptionConfig
Stores recurring payment configuration for a subscription.

```rust
pub struct SubscriptionConfig {
    pub token: Address,           // Token contract for payments
    pub amount: i128,             // Amount per interval (in token decimals)
    pub interval: u64,            // Interval duration (ledger seconds)
    pub last_charge: u64,         // Timestamp of last payment
    pub keeper_bounty: i128,      // Bounty for payment executor
}
```

**Storage Key:** `Vec<Address> = [user_address, merchant_address]`

---

## Function Reference

### 1. subscribe()

Creates a new recurring payment subscription.

#### Signature
```rust
pub fn subscribe(
    env: Env,
    user: Address,
    merchant: Address,
    token: Address,
    amount: i128,
    interval: u64,
    keeper_bounty: i128,
)
```

#### Parameters
| Parameter | Type | Description |
|-----------|------|-------------|
| `env` | Env | Soroban environment context |
| `user` | Address | Subscriber account (must authorize) |
| `merchant` | Address | Recipient merchant account |
| `token` | Address | Token contract address for payments |
| `amount` | i128 | Payment amount per interval |
| `interval` | u64 | Payment interval in ledger seconds |
| `keeper_bounty` | i128 | Bounty paid to payment executor |

#### Authorization Required
✅ `user.require_auth()`

#### Behavior
1. Validates user authorization
2. Creates composite key: `[user, merchant]`
3. Captures current ledger timestamp as `last_charge`
4. Stores SubscriptionConfig in persistent storage
5. Emits "subscribe" event

#### Events
```
Event: (symbol("subscribe"), user, merchant)
```

#### Example Usage
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source-account user1 \
  --network testnet \
  -- subscribe \
  --user GBUQWP3BOUZX34ULNQG23RQ6F4BVXEYMJUCHUNAN7GCVGQ3FJDJEWGZ \
  --merchant GBXC7EAJQGVL6KQCTFZ5GMTJKKZ3PJQIIWQV5DQVW4YYXLKWW3EQFZ \
  --token CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4 \
  --amount 1000000 \
  --interval 86400 \
  --keeper-bounty 10000
```

---

### 2. cancel()

Cancels a subscription (user-initiated).

#### Signature
```rust
pub fn cancel(
    env: Env,
    user: Address,
    merchant: Address,
)
```

#### Parameters
| Parameter | Type | Description |
|-----------|------|-------------|
| `env` | Env | Soroban environment context |
| `user` | Address | Subscriber account (must authorize) |
| `merchant` | Address | Associated merchant account |

#### Authorization Required
✅ `user.require_auth()`

#### Behavior
1. Validates user authorization
2. Creates composite key: `[user, merchant]`
3. Removes subscription from persistent storage
4. Emits "cancel" event

#### Events
```
Event: (symbol("cancel"), user, merchant)
```

#### Example Usage
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source-account user1 \
  --network testnet \
  -- cancel \
  --user GBUQWP3BOUZX34ULNQG23RQ6F4BVXEYMJUCHUNAN7GCVGQ3FJDJEWGZ \
  --merchant GBXC7EAJQGVL6KQCTFZ5GMTJKKZ3PJQIIWQV5DQVW4YYXLKWW3EQFZ
```

---

### 3. cancel_merch()

Cancels a subscription (merchant-initiated).

#### Signature
```rust
pub fn cancel_merch(
    env: Env,
    user: Address,
    merchant: Address,
)
```

#### Parameters
| Parameter | Type | Description |
|-----------|------|-------------|
| `env` | Env | Soroban environment context |
| `user` | Address | Subscriber account |
| `merchant` | Address | Merchant account (must authorize) |

#### Authorization Required
✅ `merchant.require_auth()`

#### Behavior
1. Validates merchant authorization
2. Creates composite key: `[user, merchant]`
3. Removes subscription from persistent storage
4. Emits "cancel_m" event

#### Events
```
Event: (symbol("cancel_m"), user, merchant)
```

#### Example Usage
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source-account merchant1 \
  --network testnet \
  -- cancel_merch \
  --user GBUQWP3BOUZX34ULNQG23RQ6F4BVXEYMJUCHUNAN7GCVGQ3FJDJEWGZ \
  --merchant GBXC7EAJQGVL6KQCTFZ5GMTJKKZ3PJQIIWQV5DQVW4YYXLKWW3EQFZ
```

---

### 4. execute_payment()

Executes a recurring payment when the interval has elapsed.

#### Signature
```rust
pub fn execute_payment(
    env: Env,
    user: Address,
    merchant: Address,
    keeper: Address,
) -> bool
```

#### Parameters
| Parameter | Type | Description |
|-----------|------|-------------|
| `env` | Env | Soroban environment context |
| `user` | Address | Subscriber account |
| `merchant` | Address | Recipient merchant account |
| `keeper` | Address | Payment executor (receives bounty) |

#### Returns
- `bool`: `true` on successful payment execution

#### Authorization Required
❌ None (public function)

#### Constraints
1. **Subscription Exists:** Subscription must be stored for `(user, merchant)`
2. **Time Interval Passed:** `current_ledger_timestamp >= last_charge + interval`
3. **Sufficient Balance:** User must have at least `amount + keeper_bounty` tokens

#### Behavior
1. Retrieves subscription configuration
2. Validates subscription exists (panics if not)
3. Checks time constraint: `current_time >= last_charge + interval`
4. Creates token client for subscription token
5. **First Transfer:** `amount` from user → merchant (via token contract)
6. **Second Transfer:** `keeper_bounty` from user → keeper
7. Updates `last_charge` to current ledger timestamp
8. Stores updated subscription in persistent storage
9. Emits "payment" event with timestamp
10. Returns `true`

#### Events
```
Event: (symbol("payment"), user, merchant, current_ledger_timestamp)
```

#### Possible Errors
- **"Subscription does not exist"** - Panic if subscription not found
- **"Payment is not yet due"** - Panic if interval not elapsed
- **Token Transfer Failure** - Panic if user lacks sufficient balance or token transfer fails

#### Example Usage
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source-account keeper1 \
  --network testnet \
  -- execute_payment \
  --user GBUQWP3BOUZX34ULNQG23RQ6F4BVXEYMJUCHUNAN7GCVGQ3FJDJEWGZ \
  --merchant GBXC7EAJQGVL6KQCTFZ5GMTJKKZ3PJQIIWQV5DQVW4YYXLKWW3EQFZ \
  --keeper GBKEEPEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE
```

---

### 5. get_subscription()

Queries the configuration of a subscription.

#### Signature
```rust
pub fn get_subscription(
    env: Env,
    user: Address,
    merchant: Address,
) -> Option<SubscriptionConfig>
```

#### Parameters
| Parameter | Type | Description |
|-----------|------|-------------|
| `env` | Env | Soroban environment context |
| `user` | Address | Subscriber account |
| `merchant` | Address | Merchant account |

#### Returns
- `Some(SubscriptionConfig)`: If subscription exists
- `None`: If subscription does not exist

#### Authorization Required
❌ None (read-only function)

#### Behavior
1. Creates composite key: `[user, merchant]`
2. Queries persistent storage
3. Returns SubscriptionConfig if found, None otherwise

#### Example Usage
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source-account user1 \
  --network testnet \
  -- get_subscription \
  --user GBUQWP3BOUZX34ULNQG23RQ6F4BVXEYMJUCHUNAN7GCVGQ3FJDJEWGZ \
  --merchant GBXC7EAJQGVL6KQCTFZ5GMTJKKZ3PJQIIWQV5DQVW4YYXLKWW3EQFZ
```

#### Example Response (JSON)
```json
{
  "token": "CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4",
  "amount": "1000000",
  "interval": "86400",
  "last_charge": "1234567890",
  "keeper_bounty": "10000"
}
```

---

## Event Reference

### Subscribe Event
```
Event Type: (symbol("subscribe"), Address, Address)
Payload: (symbol("subscribe"), user, merchant)
```

### Cancel Event
```
Event Type: (symbol("cancel"), Address, Address)
Payload: (symbol("cancel"), user, merchant)
```

### Cancel Merchant Event
```
Event Type: (symbol("cancel_m"), Address, Address)
Payload: (symbol("cancel_m"), user, merchant)
```

### Payment Event
```
Event Type: (symbol("payment"), Address, Address, u64)
Payload: (symbol("payment"), user, merchant, current_ledger_timestamp)
```

---

## Error Handling

### Contract Panics
- **"Payment is not yet due"** - Called when `execute_payment()` is invoked before the interval elapses
- **"Subscription does not exist"** - Called when attempting to execute payment on non-existent subscription

### Token Transfer Failures
- **Insufficient Balance** - Token contract rejects transfer if user lacks `amount + keeper_bounty`
- **Token Contract Error** - Any error from the token contract propagates as contract panic

---

## Gas Considerations

### Estimated Gas Usage (per operation)
| Operation | Approximate Gas |
|-----------|-----------------|
| `subscribe()` | 1,000-2,000 |
| `cancel()` | 500-1,000 |
| `cancel_merch()` | 500-1,000 |
| `execute_payment()` | 3,000-5,000 |
| `get_subscription()` | 100-500 |

*Note: Actual gas usage varies based on network load and contract state size*

---

## Security Best Practices

1. **Always Verify Merchant Addresses** - Ensure correct merchant before subscribing
2. **Set Reasonable Amounts** - Use subscription amounts appropriate to your use case
3. **Monitor Payment Intervals** - Track when payments execute to ensure timely fund replenishment
4. **Keeper Selection** - Only authorize trusted keepers with strong operational security
5. **Token Vetting** - Verify token contract code before subscribing to prevent transfer hooks

---

## Deployment Information

### Contract Address (Stellar Testnet)
```
CONTRACT_ID: <TO_BE_UPDATED_AFTER_DEPLOYMENT>
Network: Test Stellar Public Network ; September 2015
RPC: https://soroban-testnet.stellar.org:443
```

### Interaction Methods
1. **Soroban CLI** - Command-line interface
2. **Stellar JavaScript SDK** - Web and Node.js applications
3. **Stellar Python SDK** - Python-based tooling

---

## Reference Links

- [Soroban Documentation](https://developers.stellar.org/docs/smart-contracts)
- [Stellar Testnet Faucet](https://friendbot.stellar.org/)
- [Soroban CLI Reference](https://developers.stellar.org/docs/smart-contracts/guides/cli)

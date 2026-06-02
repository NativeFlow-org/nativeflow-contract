# NativeFlow Deployment Configuration

## Testnet Network Details
- **Network Name**: Testnet
- **Network Passphrase**: Test Stellar Public Network ; September 2015
- **Horizon RPC**: https://soroban-testnet.stellar.org:443
- **Faucet URL**: https://friendbot.stellar.org/

## Deployment Information

### Contract Details
- **Contract Name**: NativeFlow
- **Contract Type**: Soroban Smart Contract (WASM)
- **Implementation Language**: Rust
- **Soroban SDK Version**: 20.5
- **WASM Binary Size**: 5.4 KB (optimized)

### Deployment Process

1. **Prerequisites**
   - Rust 1.96+ with wasm32-unknown-unknown target
   - Soroban CLI installed (`cargo install --locked soroban-cli`)
   - Funded Stellar account on Testnet (use Friendbot faucet)

2. **Configuration**
   ```bash
   # Configure Soroban network
   soroban network add \
     --name testnet \
     --rpc-url https://soroban-testnet.stellar.org:443 \
     --network-passphrase "Test Stellar Public Network ; September 2015"
   ```

3. **Generate Deployer Identity**
   ```bash
   # Generate a new keypair for deployment
   soroban keys generate --name deployer
   
   # Fund the account using Friendbot
   curl "https://friendbot.stellar.org/?addr=<DEPLOYER_PUBLIC_KEY>"
   ```

4. **Deploy Contract**
   ```bash
   soroban contract deploy \
     --wasm target/wasm32-unknown-unknown/release/nativeflow_contract.wasm \
     --source-account deployer \
     --network testnet
   ```

5. **Capture Contract ID**
   The deployment command will return a Contract ID. Store this for future interactions:
   ```
   Contract ID: C... (Stellar contract address)
   ```

### Post-Deployment

After deployment, the contract can be invoked using:

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source-account <USER_ACCOUNT> \
  --network testnet \
  -- <function_name> \
  --<param_name> <param_value>
```

### Example: Subscribe to Payment

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source-account user1 \
  --network testnet \
  -- subscribe \
  --user <USER_ADDRESS> \
  --merchant <MERCHANT_ADDRESS> \
  --token <TOKEN_CONTRACT_ADDRESS> \
  --amount 1000000 \
  --interval 86400 \
  --keeper-bounty 10000
```

## Contract Optimization

The contract has been optimized for minimal size:
- **Optimization Level**: -O3 (release profile)
- **LTO**: Enabled
- **Codegen Units**: 1
- **Strip Symbols**: Enabled
- **Binary Size**: 5.4 KB

This ensures minimal gas costs for deployment on Stellar.

## Security

- All functions require proper authorization via `require_auth()`
- State is stored persistently using Soroban's storage API
- Token transfers use the native Soroban Token Client
- Timestamps are ledger-enforced for payment interval validation

## Testing

Before deployment to Testnet, run local tests:

```bash
# Build the contract
cargo build --target wasm32-unknown-unknown --release

# Run tests (if available)
cargo test

# Format and lint checks
cargo fmt --check
cargo clippy --lib --target wasm32-unknown-unknown -- -D warnings
```

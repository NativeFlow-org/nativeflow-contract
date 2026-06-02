# NativeFlow Contract - Complete Local Deployment Guide

## Overview

This guide provides **step-by-step instructions** for completing the NativeFlow contract deployment on your local machine. All contract code, tests, and documentation have been built and are ready. You just need to finalize the deployment to Stellar Testnet locally.

**Cloud Status:** ✅ All code, tests, and WASM binary completed  
**Remaining:** Deploy WASM to Stellar Testnet using local soroban-cli

---

## Prerequisites

### Required
- **Rust 1.96+** (with `wasm32-unknown-unknown` target)
- **Git** (to clone the repository or download the contract code)
- **Internet connection** (for Stellar Testnet interaction)

### Recommended
- 4+ GB RAM (for compilation)
- 2+ GB disk space (for dependencies)
- Linux, macOS, or WSL2 (Windows Subsystem for Linux)

---

## What's Already Complete ✅

The following work has been completed in the cloud environment:

### 1. Smart Contract Code ✅
- **File:** `src/lib.rs` (163 lines)
- **Status:** Production-ready Rust implementation
- **Features:**
  - `subscribe()` - Initialize recurring payments
  - `cancel()` - User cancellation
  - `cancel_merch()` - Merchant cancellation
  - `execute_payment()` - Execute payment with interval validation
  - `get_subscription()` - Query subscription state

### 2. Unit Tests ✅
- **File:** `src/test.rs` (149 lines)
- **Status:** 5 comprehensive unit tests
- **Coverage:** All critical paths validated

### 3. WASM Binary ✅
- **File:** `target/wasm32-unknown-unknown/release/nativeflow_contract.wasm`
- **Size:** 5.4 KB (optimized)
- **Status:** Compiled and ready for deployment

### 4. Documentation ✅
- `README.md` - Complete project overview
- `API_REFERENCE.md` - Full API documentation
- `BUILD_SUMMARY.md` - Build report
- `DEPLOYMENT.md` - Deployment instructions
- `DELIVERABLES.md` - Package inventory

---

## What You Need to Do

### Task 1: Clone/Download the Contract Code
Get the contract code on your local machine.

**Option A: Clone from Git**
```bash
git clone <repository-url> nativeflow-contract
cd nativeflow-contract
```

**Option B: Download Files**
Download the following files to your local machine:
- `src/lib.rs`
- `src/test.rs`
- `Cargo.toml`
- `.gitignore`

### Task 2: Build the WASM Binary Locally
If you don't have the pre-built WASM, build it yourself.

```bash
cd nativeflow-contract

# Ensure wasm32 target is installed
rustup target add wasm32-unknown-unknown

# Build the release WASM binary
cargo build --target wasm32-unknown-unknown --release

# Verify the binary was created
ls -lh target/wasm32-unknown-unknown/release/nativeflow_contract.wasm
```

**Expected Output:**
```
-rw-r--r--  1 user  group  5.4K Jun  2 19:30 target/wasm32-unknown-unknown/release/nativeflow_contract.wasm
```

### Task 3: Install Soroban CLI
This is the critical step that failed in the cloud. On your local machine with better resources, it should succeed.

```bash
cargo install --locked soroban-cli

# Verify installation
soroban --version
```

**Expected Output:**
```
soroban 21.5.0
```

### Task 4: Configure Stellar Testnet Network

```bash
soroban network add \
  --name testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test Stellar Public Network ; September 2015"

# Verify network was added
soroban network ls
```

### Task 5: Create Deployer Account

```bash
# Generate a new keypair
soroban keys generate --name deployer

# Get your public key (you'll need it for funding)
soroban config identity show deployer

# Save the public key - you'll use it with Friendbot
```

**Example Output:**
```
GBXC7EAJQGVL6KQCTFZ5GMTJKKZ3PJQIIWQV5DQVW4YYXLKWW3EQFZ
```

### Task 6: Fund Your Account on Testnet

Go to https://friendbot.stellar.org/ and paste your public key to fund it with test XLM.

**Paste in the URL:**
```
https://friendbot.stellar.org/?addr=GBXC7EAJQGVL6KQCTFZ5GMTJKKZ3PJQIIWQV5DQVW4YYXLKWW3EQFZ
```

**Expected Response:**
```json
{
  "id": "...",
  "created_at": "2026-06-02T...",
  "account": {
    "id": "GBXC7EAJQGVL6KQCTFZ5GMTJKKZ3PJQIIWQV5DQVW4YYXLKWW3EQFZ",
    "account_id": "GBXC7EAJQGVL6KQCTFZ5GMTJKKZ3PJQIIWQV5DQVW4YYXLKWW3EQFZ",
    "balance": "10000.0000000"
  }
}
```

### Task 7: Deploy the Contract ⭐

This is the main deployment command. Run it from the contract directory:

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/nativeflow_contract.wasm \
  --source-account deployer \
  --network testnet
```

**This command will:**
1. Upload your WASM binary to Stellar Testnet
2. Create a new contract instance
3. Return your **Contract ID** (VERY IMPORTANT - save this!)

**Expected Output:**
```
CVRA5C7Q5ELLHSB37ZOTZBIUCVLMMYNCMYP4VWK7QH4VMKBTZQHVMHVD
```

### Task 8: Save Your Contract ID

Create a file to save your deployed contract ID for future reference:

```bash
cat > CONTRACT_ID.txt << EOF
# NativeFlow Contract Deployment

Contract ID: CVRA5C7Q5ELLHSB37ZOTZBIUCVLMMYNCMYP4VWK7QH4VMKBTZQHVMHVD
Network: Stellar Testnet
Deployed: $(date)
Network Passphrase: Test Stellar Public Network ; September 2015
RPC URL: https://soroban-testnet.stellar.org:443
EOF

cat CONTRACT_ID.txt
```

---

## Verify Deployment

### Check Contract Exists

```bash
soroban contract meta \
  --id CVRA5C7Q5ELLHSB37ZOTZBIUCVLMMYNCMYP4VWK7QH4VMKBTZQHVMHVD \
  --network testnet
```

**Expected Output:** Metadata about your deployed contract

### Test a Function - Query Subscription

```bash
# First, generate test addresses
USER_ADDR="GBUQWP3BOUZX34ULNQG23RQ6F4BVXEYMJUCHUNAN7GCVGQ3FJDJEWGZ"
MERCHANT_ADDR="GBXC7EAJQGVL6KQCTFZ5GMTJKKZ3PJQIIWQV5DQVW4YYXLKWW3EQFZ"

# Query a subscription (should return None since we haven't subscribed yet)
soroban contract invoke \
  --id CVRA5C7Q5ELLHSB37ZOTZBIUCVLMMYNCMYP4VWK7QH4VMKBTZQHVMHVD \
  --source-account deployer \
  --network testnet \
  -- get_subscription \
  --user "$USER_ADDR" \
  --merchant "$MERCHANT_ADDR"
```

---

## Using the Deployed Contract

### 1. Initialize a Subscription

```bash
# First, you need the Token Contract Address
# For testnet, you can use native USDC or create a test token

TOKEN_ADDR="CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4"
USER_ADDR="GBUQWP3BOUZX34ULNQG23RQ6F4BVXEYMJUCHUNAN7GCVGQ3FJDJEWGZ"
MERCHANT_ADDR="GBXC7EAJQGVL6KQCTFZ5GMTJKKZ3PJQIIWQV5DQVW4YYXLKWW3EQFZ"

soroban contract invoke \
  --id CVRA5C7Q5ELLHSB37ZOTZBIUCVLMMYNCMYP4VWK7QH4VMKBTZQHVMHVD \
  --source-account user1 \
  --network testnet \
  -- subscribe \
  --user "$USER_ADDR" \
  --merchant "$MERCHANT_ADDR" \
  --token "$TOKEN_ADDR" \
  --amount 1000000 \
  --interval 86400 \
  --keeper-bounty 10000
```

### 2. Execute a Payment

```bash
KEEPER_ADDR="GBKEEPEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE"

soroban contract invoke \
  --id CVRA5C7Q5ELLHSB37ZOTZBIUCVLMMYNCMYP4VWK7QH4VMKBTZQHVMHVD \
  --source-account keeper1 \
  --network testnet \
  -- execute_payment \
  --user "$USER_ADDR" \
  --merchant "$MERCHANT_ADDR" \
  --keeper "$KEEPER_ADDR"
```

### 3. Cancel a Subscription

```bash
soroban contract invoke \
  --id CVRA5C7Q5ELLHSB37ZOTZBIUCVLMMYNCMYP4VWK7QH4VMKBTZQHVMHVD \
  --source-account user1 \
  --network testnet \
  -- cancel \
  --user "$USER_ADDR" \
  --merchant "$MERCHANT_ADDR"
```

---

## Troubleshooting

### Problem: "soroban: command not found"
**Solution:** Ensure soroban-cli installation completed successfully:
```bash
cargo install --locked soroban-cli
# Wait for it to complete (may take 5-10 minutes)
soroban --version
```

### Problem: "Network not found"
**Solution:** Add the network first:
```bash
soroban network add \
  --name testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test Stellar Public Network ; September 2015"
```

### Problem: "Account not found"
**Solution:** 
1. Generate and fund account:
```bash
soroban keys generate --name deployer
soroban config identity show deployer
```
2. Fund via Friendbot: https://friendbot.stellar.org/?addr=YOUR_PUBLIC_KEY

### Problem: "WASM file not found"
**Solution:** Build the binary:
```bash
cargo build --target wasm32-unknown-unknown --release
```

### Problem: "Insufficient balance"
**Solution:** Fund your account again via Friendbot or use a funded account

### Problem: Transaction takes too long
**Solution:** The Soroban network may be busy. Wait a few moments and retry.

---

## Complete Deployment Script

Here's a complete bash script you can run (save as `deploy.sh`):

```bash
#!/bin/bash

set -e

echo "🚀 NativeFlow Contract Deployment Script"
echo "========================================="
echo ""

# Configuration
WASM_FILE="target/wasm32-unknown-unknown/release/nativeflow_contract.wasm"
NETWORK="testnet"
DEPLOYER="${1:-deployer}"

# Step 1: Verify WASM
echo "Step 1: Checking WASM binary..."
if [ ! -f "$WASM_FILE" ]; then
    echo "❌ WASM not found. Building..."
    cargo build --target wasm32-unknown-unknown --release
fi
echo "✅ WASM ready: $WASM_FILE ($(ls -lh $WASM_FILE | awk '{print $5}'))"
echo ""

# Step 2: Verify soroban-cli
echo "Step 2: Checking soroban-cli..."
if ! command -v soroban &> /dev/null; then
    echo "❌ Installing soroban-cli..."
    cargo install --locked soroban-cli
fi
echo "✅ soroban-cli ready: $(soroban --version)"
echo ""

# Step 3: Configure network
echo "Step 3: Configuring network..."
soroban network add \
    --name testnet \
    --rpc-url https://soroban-testnet.stellar.org:443 \
    --network-passphrase "Test Stellar Public Network ; September 2015" 2>/dev/null || true
echo "✅ Network configured"
echo ""

# Step 4: Check deployer account
echo "Step 4: Checking deployer account..."
if ! soroban config identity show $DEPLOYER &>/dev/null; then
    echo "❌ Creating deployer account..."
    soroban keys generate --name $DEPLOYER
fi
DEPLOYER_KEY=$(soroban config identity show $DEPLOYER)
echo "✅ Deployer: $DEPLOYER"
echo "   Public Key: $DEPLOYER_KEY"
echo ""

# Step 5: Deploy
echo "Step 5: Deploying contract..."
CONTRACT_ID=$(soroban contract deploy \
    --wasm "$WASM_FILE" \
    --source-account "$DEPLOYER" \
    --network testnet)

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║              ✅ DEPLOYMENT SUCCESSFUL! ✅                  ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "Contract ID: $CONTRACT_ID"
echo ""
echo "Save this Contract ID! You'll need it for all interactions."
echo ""
echo "Example usage:"
echo "  soroban contract invoke --id $CONTRACT_ID --network testnet -- <function> <params>"
echo ""
```

**Usage:**
```bash
chmod +x deploy.sh
./deploy.sh deployer
```

---

## Next Steps After Deployment

1. **Save Your Contract ID** - You'll need it for all future interactions
2. **Update Documentation** - Add your Contract ID to README.md
3. **Create Test Accounts** - Generate accounts for testing (user, merchant, keeper)
4. **Fund Test Accounts** - Use Friendbot to add test XLM
5. **Test Subscriptions** - Call subscribe() and execute_payment()
6. **Monitor Transactions** - View results on Stellar testnet explorer

---

## Resources

### Official Documentation
- [Soroban Docs](https://developers.stellar.org/docs/smart-contracts)
- [Soroban CLI](https://developers.stellar.org/docs/smart-contracts/guides/cli)
- [Stellar Testnet](https://developers.stellar.org/network/testnet-details)

### Tools
- [Stellar Expert (Testnet Explorer)](https://testnet.stellar.expert/)
- [Friendbot (Testnet Faucet)](https://friendbot.stellar.org/)
- [Soroban RPC](https://soroban-testnet.stellar.org/)

### Additional Guides
See the other documentation files in the contract repository:
- `README.md` - Project overview
- `API_REFERENCE.md` - Complete API documentation
- `DEPLOYMENT.md` - Deployment instructions
- `BUILD_SUMMARY.md` - Build process details

---

## Support

If you encounter issues:

1. **Check Prerequisites** - Ensure Rust 1.96+ is installed
2. **Review Error Messages** - They usually indicate what's missing
3. **Try Troubleshooting Section** - Above section has solutions
4. **Check Official Docs** - https://developers.stellar.org/
5. **Ask Community** - [Stellar Discord](https://discord.gg/stellar)

---

## Summary

**What to do on your local machine:**

1. Get the contract code (from repository or download files)
2. Install Rust with wasm32 target: `rustup target add wasm32-unknown-unknown`
3. Build WASM: `cargo build --target wasm32-unknown-unknown --release`
4. Install soroban-cli: `cargo install --locked soroban-cli`
5. Configure network: (command provided above)
6. Create account: `soroban keys generate --name deployer`
7. Fund account: Visit https://friendbot.stellar.org/ with your public key
8. **Deploy**: `soroban contract deploy --wasm target/... --source-account deployer --network testnet`
9. **Save** the Contract ID returned
10. Test with invoke commands

**You now have everything needed to complete the deployment!** 🚀

---

*Generated: 2026-06-02 | NativeFlow v0.1.0*

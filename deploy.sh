#!/bin/bash
# NativeFlow Contract Deployment Script
# This script demonstrates the deployment process for the NativeFlow contract

set -e

# Configuration
WASM_FILE="target/wasm32-unknown-unknown/release/nativeflow_contract.wasm"
NETWORK="testnet"
DEPLOYER_ACCOUNT="${DEPLOYER_ACCOUNT:-deployer}"

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║         NativeFlow Contract Deployment Script                 ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# Verify WASM file exists
if [ ! -f "$WASM_FILE" ]; then
    echo "❌ ERROR: WASM file not found at $WASM_FILE"
    echo "Please build the contract first:"
    echo "  cargo build --target wasm32-unknown-unknown --release"
    exit 1
fi

echo "✅ WASM file found: $WASM_FILE"
echo "   Size: $(ls -lh $WASM_FILE | awk '{print $5}')"
echo ""

# Step 1: Verify soroban-cli installation
echo "Step 1: Verifying soroban-cli..."
if ! command -v soroban &> /dev/null; then
    echo "❌ soroban-cli not found!"
    echo ""
    echo "To install soroban-cli, run:"
    echo "  cargo install --locked soroban-cli"
    echo ""
    echo "Or download pre-built binary from:"
    echo "  https://github.com/stellar/soroban-cli/releases"
    exit 1
fi

SOROBAN_VERSION=$(soroban --version)
echo "✅ soroban-cli installed: $SOROBAN_VERSION"
echo ""

# Step 2: Configure network
echo "Step 2: Configuring Stellar Testnet network..."
soroban network add \
    --name testnet \
    --rpc-url https://soroban-testnet.stellar.org:443 \
    --network-passphrase "Test Stellar Public Network ; September 2015" 2>/dev/null || true

echo "✅ Network configured"
echo ""

# Step 3: Verify deployer account
echo "Step 3: Verifying deployer account..."
if ! soroban config identity show $DEPLOYER_ACCOUNT &>/dev/null; then
    echo "❌ Deployer account '$DEPLOYER_ACCOUNT' not found!"
    echo ""
    echo "To create a deployer account:"
    echo "  soroban keys generate --name $DEPLOYER_ACCOUNT"
    echo ""
    echo "To fund the account, visit:"
    echo "  https://friendbot.stellar.org/?addr=<YOUR_PUBLIC_KEY>"
    exit 1
fi

DEPLOYER_KEY=$(soroban config identity show $DEPLOYER_ACCOUNT)
echo "✅ Deployer account: $DEPLOYER_ACCOUNT"
echo "   Public Key: $DEPLOYER_KEY"
echo ""

# Step 4: Deploy contract
echo "Step 4: Deploying NativeFlow contract to Stellar Testnet..."
echo "   This may take a moment..."
echo ""

CONTRACT_ID=$(soroban contract deploy \
    --wasm "$WASM_FILE" \
    --source-account "$DEPLOYER_ACCOUNT" \
    --network testnet)

echo "✅ CONTRACT DEPLOYED SUCCESSFULLY!"
echo ""
echo "╔════════════════════════════════════════════════════════════════╗"
echo "║                    DEPLOYMENT COMPLETE                        ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""
echo "📋 Contract Information:"
echo "   Network:     Stellar Testnet"
echo "   Contract ID: $CONTRACT_ID"
echo "   RPC URL:     https://soroban-testnet.stellar.org:443"
echo ""
echo "💾 Save this Contract ID for future interactions!"
echo ""
echo "Next: You can now invoke contract functions using:"
echo "   soroban contract invoke \\"
echo "     --id $CONTRACT_ID \\"
echo "     --source-account $DEPLOYER_ACCOUNT \\"
echo "     --network testnet \\"
echo "     -- <function_name> <params>"
echo ""
echo "Example - Subscribe to payment:"
echo "   soroban contract invoke \\"
echo "     --id $CONTRACT_ID \\"
echo "     --source-account user1 \\"
echo "     --network testnet \\"
echo "     -- subscribe \\"
echo "     --user <USER_ADDRESS> \\"
echo "     --merchant <MERCHANT_ADDRESS> \\"
echo "     --token <TOKEN_ADDRESS> \\"
echo "     --amount 1000000 \\"
echo "     --interval 86400 \\"
echo "     --keeper-bounty 10000"
echo ""

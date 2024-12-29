#!/bin/bash
set -e

# Load constants
CONSTANTS_FILE=scripts/messages/constants.json
if [ ! -f "$CONSTANTS_FILE" ]; then
  echo "Error: Constants file not found at $CONSTANTS_FILE"
  exit 1
fi
eval "$(jq -r 'to_entries | .[] | "export \(.key)=\(.value)"' "$CONSTANTS_FILE")"

# Load state from previous step
if [ ! -f "scripts/state/03_create_minter.state" ]; then
  echo "Error: Previous state file not found"
  exit 1
fi

# Get minter contract address
MINTER_CONTRACT=$(grep '^minter_contract=' scripts/state/03_create_minter.state | cut -d= -f2)
if [ -z "$MINTER_CONTRACT" ]; then
  echo "Error: Minter contract address not found in state file"
  exit 1
fi

# Get deployer address
if [ -z "$DEPLOYER_ADDRESS" ]; then
  echo "Error: DEPLOYER_ADDRESS not set"
  exit 1
fi

echo "Minting token..."
echo "Minter contract: $MINTER_CONTRACT"
echo "Owner: $DEPLOYER_ADDRESS"
echo "Amount: $MINT_PRICE$TOKEN_DENOM"

# Execute mint with more gas
TX_HASH=$(starsd tx wasm execute "$MINTER_CONTRACT" '{"mint":{}}' \
  --from "$DEPLOYER_ADDRESS" \
  --amount "$MINT_PRICE$TOKEN_DENOM" \
  --gas-prices "$GAS_PRICE$TOKEN_DENOM" \
  --gas-adjustment "1.9" \
  --gas "500000" \
  --broadcast-mode "$BROADCAST_MODE" \
  --chain-id "$CHAIN_ID" \
  --node "$NODE_URL" \
  --output json \
  -y | jq -r '.txhash')

echo "Transaction hash: $TX_HASH"

# Save state
STATE_FILE="scripts/state/04_mint_token.state"
mkdir -p "$(dirname "$STATE_FILE")"
echo "tx_hash=$TX_HASH" > "$STATE_FILE"

echo "Done! Token minted successfully." 
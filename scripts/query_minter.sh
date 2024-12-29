#!/bin/bash
set -e

source scripts/00_load_constants.sh

# Load minter address from state
if [ ! -f "scripts/state/03_create_minter.state" ]; then
    echo -e "\033[0;31mMinter state file not found\033[0m"
    exit 1
fi

# Get the labeled addresses from state
MINTER_CONTRACT=$(grep "^minter_contract=" "scripts/state/03_create_minter.state" | cut -d'=' -f2)
if [ -z "$MINTER_CONTRACT" ]; then
    echo -e "\033[0;31mMinter contract address not found in state\033[0m"
    exit 1
fi

SG721_CONTRACT=$(grep "^sg721_contract=" "scripts/state/03_create_minter.state" | cut -d'=' -f2)
if [ -z "$SG721_CONTRACT" ]; then
    echo -e "\033[0;31mSG721 contract address not found in state\033[0m"
    exit 1
fi

echo "Minter contract: $MINTER_CONTRACT"
echo "SG721 contract: $SG721_CONTRACT"

# Query minter config
echo -e "\nMinter config:"
starsd query wasm contract-state smart "$MINTER_CONTRACT" '{"config":{}}' --node "$NODE_URL" --output json | jq .

# Query minter start time
echo -e "\nMinter start time:"
starsd query wasm contract-state smart "$MINTER_CONTRACT" '{"start_time":{}}' --node "$NODE_URL" --output json | jq .

# Query minter status
echo -e "\nMinter status:"
starsd query wasm contract-state smart "$MINTER_CONTRACT" '{"status":{}}' --node "$NODE_URL" --output json | jq .

# Query mint price
echo -e "\nMint price:"
starsd query wasm contract-state smart "$MINTER_CONTRACT" '{"mint_price":{}}' --node "$NODE_URL" --output json | jq .

# Query mint count
echo -e "\nMint count:"
starsd query wasm contract-state smart "$MINTER_CONTRACT" '{"mint_count":{}}' --node "$NODE_URL" --output json | jq .

# Query mintable num tokens
echo -e "\nMintable num tokens:"
starsd query wasm contract-state smart "$MINTER_CONTRACT" '{"mintable_num_tokens":{}}' --node "$NODE_URL" --output json | jq .

# Query collection info from sg721
echo -e "\nCollection info:"
starsd query wasm contract-state smart "$SG721_CONTRACT" '{"collection_info":{}}' --node "$NODE_URL" --output json | jq . 
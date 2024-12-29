#!/bin/bash
set -e

source scripts/00_load_constants.sh

# Load sg721 address from state
if [ ! -f "scripts/state/03_create_minter.state" ]; then
    echo -e "\033[0;31mMinter state file not found\033[0m"
    exit 1
fi

SG721_CONTRACT=$(grep "^sg721_contract=" "scripts/state/03_create_minter.state" | cut -d'=' -f2)
if [ -z "$SG721_CONTRACT" ]; then
    echo -e "\033[0;31mSG721 contract address not found in state\033[0m"
    exit 1
fi

echo "SG721 contract: $SG721_CONTRACT"

# Query all tokens
echo -e "\nAll tokens:"
starsd query wasm contract-state smart "$SG721_CONTRACT" '{"all_tokens":{"start_after":null,"limit":null}}' --node "$NODE_URL" --output json | jq .

# Query number of tokens
echo -e "\nNumber of tokens:"
starsd query wasm contract-state smart "$SG721_CONTRACT" '{"num_tokens":{}}' --node "$NODE_URL" --output json | jq . 
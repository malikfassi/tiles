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

# Check if token ID is provided
if [ -z "$1" ]; then
    echo -e "\033[0;31mPlease provide a token ID\033[0m"
    echo "Usage: $0 <token_id>"
    exit 1
fi

TOKEN_ID=$1
echo "SG721 contract: $SG721_CONTRACT"
echo "Token ID: $TOKEN_ID"

# Query token info
echo -e "\nToken info:"
starsd query wasm contract-state smart "$SG721_CONTRACT" '{"nft_info":{"token_id":"'$TOKEN_ID'"}}' --node "$NODE_URL" --output json | jq .

# Query all token info (includes approvals)
echo -e "\nAll token info:"
starsd query wasm contract-state smart "$SG721_CONTRACT" '{"all_nft_info":{"token_id":"'$TOKEN_ID'"}}' --node "$NODE_URL" --output json | jq . 
#!/bin/bash
# Reads the deployed collection: config, price grid and one tile's pixels.
#
# Usage:
#   scripts/query_contract.sh                 # config + price grid
#   scripts/query_contract.sh <token_id>      # adds the pixels of that tile
set -e

source scripts/00_load_constants.sh

STATE_FILE="scripts/state/02_deploy_contracts.state"
if [ ! -f "$STATE_FILE" ]; then
    echo -e "\033[0;31mNo deployment state found: run scripts/02_deploy_contracts.sh first\033[0m"
    exit 1
fi

TILE_CONTRACT=$(grep "^tile_contract=" "$STATE_FILE" | cut -d'=' -f2)
echo -e "\033[0;34mContract: $TILE_CONTRACT on $CHAIN_ID\033[0m"

query() {
    gaiad query wasm contract-state smart "$TILE_CONTRACT" "$1" \
        --node "$NODE_URL" --output json | jq '.data'
}

echo -e "\n\033[0;34m--- Config (payment split, price floor) ---\033[0m"
query '{"extension":{"msg":{"config":{}}}}'

echo -e "\n\033[0;34m--- Price grid ---\033[0m"
query '{"extension":{"msg":{"price_scaling":{}}}}'

echo -e "\n\033[0;34m--- Collection info (royalties) ---\033[0m"
query '{"collection_info":{}}'

if [ -n "$1" ]; then
    echo -e "\n\033[0;34m--- Pixels of tile $1 ---\033[0m"
    query "{\"extension\":{\"msg\":{\"tile_pixels\":{\"token_id\":\"$1\"}}}}"
fi

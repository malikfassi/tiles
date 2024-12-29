#!/bin/bash
set -e

source scripts/00_load_constants.sh

# Load factory address from state
if [ ! -f "scripts/state/02_deploy_contracts.state" ]; then
    echo -e "\033[0;31mFactory state file not found\033[0m"
    exit 1
fi

FACTORY_CONTRACT=$(grep "^factory_contract=" "scripts/state/02_deploy_contracts.state" | cut -d'=' -f2 | grep -o 'stars[a-zA-Z0-9]*')
if [ -z "$FACTORY_CONTRACT" ]; then
    echo -e "\033[0;31mFactory contract address not found in state\033[0m"
    exit 1
fi

starsd query wasm contract-state smart "$FACTORY_CONTRACT" '{"params":{}}' --node "$NODE_URL" --output json | jq . 
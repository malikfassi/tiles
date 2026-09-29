#!/bin/bash
# Shows a transaction's result, including the contract events an indexer would read.
#
# Usage: scripts/query_tx.sh <txhash>
set -e

if [ $# -lt 1 ]; then
    echo "Usage: $0 <txhash>"
    exit 1
fi

source scripts/00_load_constants.sh

gaiad query tx "$1" --node "$NODE_URL" --output json \
    | jq '{code, raw_log, gas_used: .gas_used, events: [.events[] | select(.type | startswith("wasm"))]}'

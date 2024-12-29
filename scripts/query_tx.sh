#!/bin/bash
set -e

source scripts/00_load_constants.sh

# Get tx hash from argument
TX_HASH=$1

starsd query tx "$TX_HASH" --node "$NODE_URL" --output json | jq . 
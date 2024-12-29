#!/bin/bash
set -x

# Load constants from JSON file
CONSTANTS_FILE="scripts/messages/constants.json"
if [ ! -f "$CONSTANTS_FILE" ]; then
    echo "Constants file not found. Run 'cargo build' first to generate it."
    exit 1
fi

# Export each constant as an environment variable
eval $(jq -r 'to_entries | .[] | "export \(.key)=\(.value)"' "$CONSTANTS_FILE") 
#!/bin/bash

# Get the directory of the script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
CLIENT_KEY="12345678901234567890123456789010"
E2E_KEY="12345678901234567890123456789013"
COLLECTOR_ADDRESS="127.0.0.1:12999"

CLIENT_KEY=$CLIENT_KEY E2E_KEY=$E2E_KEY COLLECTOR_ADDRESS=$COLLECTOR_ADDRESS $SCRIPT_DIR/../target/release/sf-ope-client

# Check if the application exited successfully
if [ $? -ne 0 ]; then
    echo "Error: Failed to run the compiled application, did you run ./build.sh?"
    exit 1
fi
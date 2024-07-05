#!/bin/bash

# Get the directory of the script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
COLLECTOR_KEY="12345678901234567890123456789012"
E2E_KEY="12345678901234567890123456789013"

# Check if the correct number of arguments is provided
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <sensor_id> <faulty>"
    exit 1
fi

sensor_id=$1
faulty=$2

if ! [[ "$sensor_id" =~ ^[0-9]+$ ]]; then
    echo "Error: the sensor id must be a valid integer"
    exit 1
fi

if [ "$faulty" != "true" ] && [ "$faulty" != "false" ]; then
    echo "Error: The faulty status must be 'true' or 'false'."
    exit 1
fi


START_PORT=13300
PORT=$((START_PORT + sensor_id)) COLLECTOR_KEY=$COLLECTOR_KEY E2E_KEY=$E2E_KEY FAULTY=$faulty $SCRIPT_DIR/../target/release/sf-ope-sensor


# Check if the application exited successfully
if [ $? -ne 0 ]; then
    echo "Error: Failed to run the compiled application, did you run ./build.sh?"
    exit 1
fi



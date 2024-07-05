#!/bin/bash

# Get the directory of the script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

# Check the number of arguments
if [ $# -ne 1 ]; then
    echo "Usage: $0 <number_of_sensors>"
    exit 1
fi

# Number of sensors
NUM_SENSORS=$1

# Initialize SENSOR_ADDRESSES and SENSOR_KEYS variable
SENSOR_ADDRESSES=""
SENSOR_KEYS=""
CLIENT_KEY="12345678901234567890123456789010"

# Construct SENSOR_ADDRESSES based on the number of sensors
for (( i=1; i<=$NUM_SENSORS; i++ )); do
    PORT=$((13300 + i))
    SENSOR_ADDRESSES+="127.0.0.1:$PORT"
    SENSOR_KEYS+="12345678901234567890123456789012"
    if [ $i -lt $NUM_SENSORS ]; then
        SENSOR_ADDRESSES+=","
        SENSOR_KEYS+=","
    fi
done

# Run collector
SENSOR_ADDRESSES=$SENSOR_ADDRESSES SENSOR_KEYS=$SENSOR_KEYS CLIENT_KEY=$CLIENT_KEY $SCRIPT_DIR/../target/release/sf-ope-sensorcollector

# Check if the application exited successfully
if [ $? -ne 0 ]; then
    echo "Error: Failed to run the compiled application, did you run ./build.sh?"
    exit 1
fi

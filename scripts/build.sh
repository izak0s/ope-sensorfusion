#!/bin/bash

# Get the directory of the script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

# Move to the parent directory
cd $SCRIPT_DIR/..

# Compile the project
echo "Compiling Rust project..."
cargo build --release

# Check if compilation was successful
if [ $? -eq 0 ]; then
    echo "Rust project compiled successfully."
else
    echo "Error: Failed to compile Rust project."
    exit 1
fi


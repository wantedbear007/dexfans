
#!/bin/bash
set -e  

# checking blank args
if [ "$#" -eq 0 ]; then
    echo "Error: No canister name provided."
    echo "Usage: $0 <canister_name>"
    exit 1
fi

canister_name=$1

# Check if candid-extractor is already installed
if ! command -v candid-extractor &> /dev/null; then
    echo "Installing candid-extractor..."
    cargo install candid-extractor
fi

# Build the canister
cargo build --release --target wasm32-unknown-unknown --package "$canister_name"

# Paths
wasm_path="./target/wasm32-unknown-unknown/release/${canister_name}.wasm"
did_output="./src/backend/${canister_name}/${canister_name}.did"

# Generate DID file
candid-extractor "$wasm_path" > "$did_output"

echo "DID file generated at: $did_output"

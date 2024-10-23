#!/bin/bash

# Configs
# local rust canisters 
# canisters=("dexfans_backend" "post_canister")


# chmod 777 ./pratap_scripts/main.sh
# ./pratap_scripts/main.sh $canisters

canisters=("post_canister" "dexfans_backend")

./generate_did.sh
# Define the path to the script
script_path="./scripts/main.sh"

# Check if the script exists
if [ ! -f "$script_path" ]; then
    echo "Error: $script_path not found!"
    exit 1
fi

# Make the script executable only if it's not already executable
if [ ! -x "$script_path" ]; then
    chmod 777 "$script_path"
fi

# Execute the script with the canisters as arguments
"$script_path" "${canisters[@]}"





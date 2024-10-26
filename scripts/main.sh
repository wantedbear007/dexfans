


#!/bin/bash

set -e 


canisters=("post_canister" "index_canister")
frontend_canister=("")


# Function to generate DID
generate_did() {
    for element in "$@"; do
        ./scripts/deps/generate_did.sh "$element"
        echo "Generated DID for: $element"
    done
}

# Function to generate DID and deploy locally
generate_did_and_deploy_local() {
    generate_did "$@"
    chmod 777 ./scripts/deployment/local_deployment.sh
    for element in "$@"; do
        ./scripts/deployment/local_deployment.sh "$element"
        echo "Deployed all cainsters locally."
    done
}

# Function to generate DID and deploy to main network
generate_did_and_deploy_main() {
  chmod 777 ./scripts/deployment/main_deployment.sh
    generate_did "$@"
    for element in "$@"; do
        ./scripts/deployment/main_deployment.sh "$element"
        echo "Deployed all to the main network."
    done
}

# Function to build all canisters 
build_and_create_all_cainsters() {
    for element in "$@"; do
        dfx canister create "$element"
    done
    for element in "$@"; do
        dfx canister create "$element"
    done

    echo "Build success."

}

# Function to start a clean instance and deploy locally
start_clean_instance_and_deploy() {
    dfx stop
    dfx start --clean --background
    generate_did_and_deploy_local "$@"
}

# User prompt for action
echo "What would you like to do?"
echo "1. Generate DIDs and deploy locally"
echo "2. Start a clean instance and deploy locally"
echo "3. Generate DIDs and deploy to the main network"
echo "4. Create and build canisters"
read -p "Enter your choice (1/2/3/4): " choice

# Handle user choice
case $choice in
    1)
        generate_did_and_deploy_local "$canisters"
        ;;
    2)
        start_clean_instance_and_deploy "$canisters"
        ;;
    3)
        generate_did_and_deploy_main "$canisters"
        ;;
    4)
        build_and_create_all_cainsters "$canisters"
        ;;
    *)
        echo "Invalid choice. Please enter 1, 2, or 3."
        exit 1
        ;;
esac

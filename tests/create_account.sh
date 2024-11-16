#!/bin/bash

set -e

# Number of users you want to register
NUM_USERS=20
START_NUM=0
echo "Creating User Identities..."

CANISTER=$(dfx canister id index_canister)
echo "Canister ID: $CANISTER"

declare -a interests=("Gaming" "Art" "Technology" "Finance" "Sports")

# Loop to register multiple users
for i in $(seq $START_NUM $NUM_USERS); do
    dfx identity new "user$i" --storage-mode=plaintext || true
    echo "User identity user$i created."

    # Fetch random data from the API for variety in usernames and biographies
    response=$(curl -s "https://randomuser.me/api/?nat=us,ca,gb,au,nz")
    
    # Extracting username from the API, you might want to add a prefix to ensure uniqueness
    USERNAME=$(jq -r '.results[0].login.username' <<< "$response")
    BIO="hello from $USERNAME, interested in ${interests[$i % ${#interests[@]}]}."

    COVER_IMAGE="1"
    AVATAR="1"

    echo "Using identity user$i"
    dfx identity use "user$i"
    CURRENT_PRINCIPAL=$(dfx identity get-principal)
    echo "Current Principal: $CURRENT_PRINCIPAL"

    RES=$(dfx canister call $CANISTER api_create_account "(
      record {
        username = \"$USERNAME\";
        bio = opt \"$BIO\";
        cover_image = opt \"$COVER_IMAGE\";
        avatar = opt \"$AVATAR\";
      }
    )")

    echo "Response: $RES"
done

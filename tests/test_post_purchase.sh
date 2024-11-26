#!/bin/bash
set -e  # Exit the script if any command fails

# NOTE: If you're using macOS and encounter errors, switch from zsh to bash terminal.

# Retrieve principal IDs for various identities
DEFAULT=$(dfx --identity default identity get-principal)       # Principal ID for the default identity
RECIEVER=$(dfx --identity Bhanu identity get-principal)       # Principal ID for the Bhanu identity
SENDER=$(dfx --identity minter identity get-principal)        # Principal ID for the minter identity
LOL=$(dfx --identity lol identity get-principal)        # Principal ID for the minter identity

# Retrieve the canister ID for the index_canister
INDEX_CANISTER=$(dfx canister id index_canister)

# Print principal details for debugging
echo "DEFAULT: $DEFAULT"  # Principal of the default identity
echo "USER: $RECIEVER"    # Principal of the receiver identity

# Function to print balances for debugging
function debug_print() {
    echo "Balance of default: $(dfx canister call icp_ledger_canister icrc1_balance_of "(record {owner = principal \"$DEFAULT\"})")"
    echo "Balance of sender: $(dfx canister call icp_ledger_canister icrc1_balance_of "(record {owner = principal \"$SENDER\"})")"
    echo "Balance of receiver: $(dfx canister call icp_ledger_canister icrc1_balance_of "(record {owner = principal \"$RECIEVER\"})")"
        echo "Balance of lol: $(dfx canister call icp_ledger_canister icrc1_balance_of "(record {owner = principal \"$LOL\"})")"
}

# Perform a dummy ICP transfer (for testing purposes only)
# NOTE: This won't work in production and is only for test environments.
DUMMY_ICP_TRANSFER=$(
    dfx --identity default canister call icp_ledger_canister icrc1_transfer "(record { to = record { owner = principal \"$SENDER\" }; amount = 1000000000 })"
)
echo "Dummy ICP Transfer Result: $DUMMY_ICP_TRANSFER"

# Approve the index_canister to spend a specific amount on behalf of the minter identity
# NOTE: In a production environment, this approval step should be performed by the frontend.
APPROVE=$(
    dfx --identity minter canister call icp_ledger_canister icrc2_approve "(record { amount = 20000000; spender = record { owner = principal \"$INDEX_CANISTER\"} })"
)
echo "Approval Result: $APPROVE"

# # Transfer a membership to the receiver
# # Here, a purchase of a "Silver" membership variant is initiated.
# USER_TRANSFER=$(
#     dfx --identity minter canister call index_canister api_purchase_post "(variant { Diamond })"
# )
# echo "User Membership Purchase Result: $USER_TRANSFER"

# Debug balances for verification
debug_print

# End of script

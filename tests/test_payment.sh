set -e

# IMPORTANT: On mac if error occurs use bash terminal instead of zsh.
DEFAULT=$(dfx --identity default identity get-principal)
RECIEVER=$(dfx --identity Bhanu identity get-principal)
SENDER=$(dfx --identity minter identity get-principal)

INDEX_CANISTER=$(dfx canister id index_canister)

echo "DEFAULT: $DEFAULT"
echo "USER: $RECIEVER"

function debug_print() {

    echo "Balance of default: $(dfx canister call icp_ledger_canister icrc1_balance_of "(record {owner = principal \"$DEFAULT\"})")"

    echo "Balance of sender: $(dfx canister call icp_ledger_canister icrc1_balance_of "(record {owner = principal \"$SENDER\"})")"

    echo "Balance of reciever: $(dfx canister call icp_ledger_canister icrc1_balance_of "(record {owner = principal \"$RECIEVER\"})")"
    


}

# # TRANSFER (For testing purpose only won't work in production)
DUMMY_ICP_TRANSFER=$(
dfx --identity default canister call icp_ledger_canister icrc1_transfer "(record { to = record { owner = principal \"$SENDER\" }; amount = 1000000000 })")
echo $DUMMY_ICP_TRANSFER


# # to approve (in production frontend should perform this)
APPROVE=$(dfx --identity minter canister call icp_ledger_canister icrc2_approve "(record { amount = 20000000; spender = record { owner = principal \"$INDEX_CANISTER\"} })")
echo $APPROVE

# # # TRANSFER TO USER
# USER_TRANSFER=$(dfx --identity minter canister call index_canister api_complete_payment "(10000, principal \"$SENDER\")")
# echo $USER_TRANSFER

debug_print 

# (variant { Silver })
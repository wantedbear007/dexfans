
set -e

# Controller ids
# dfx identity new Bhanu
# dfx identity use Bhanu
dfx identity new minter --storage-mode=plaintext || true
dfx identity new reciever --storage-mode=plaintext || true
dfx identity new Bhanu --storage-mode=plaintext || true


BHANU=$(dfx identity get-principal --identity Bhanu)

# dfx deps init
# dfx deps deploy 
# dfx deps pull
# for creating canisters IDS
# dfx canister create icp_ledger_canister
dfx canister create asset_handler --ic
dfx canister create post_canister --ic
dfx canister create index_canister --ic

# for compiling canisters
# dfx build icp_ledger_canister
dfx build asset_handler --ic
dfx build post_canister --ic
dfx build index_canister --ic

# Canister IDS
LEDGER_CANISTER="ryjl3-tyaaa-aaaaa-aaaba-cai"
IC_ASSET_CANISTER=$(dfx canister id asset_handler --ic)
INDEX_CANISTER=$(dfx canister id index_canister --ic)
POST_CANISTER=$(dfx canister id post_canister --ic)
FRONTEND_CANISTER=$(dfx canister id post_canister --ic)


# FOR ICP LEDGER
MINTER_ACCOUNT_ID=$(dfx --identity anonymous ledger account-id)
DEFAULT_ACCOUNT_ID=$(dfx --identity default ledger account-id)


# IMP: Review below warnings
# Update code in /src/index_canister/src/lib.rs if below keys are changed 


dfx deploy index_canister --argument "( record {
    active_post_canister = principal \"${POST_CANISTER}\";
    payment_recipient = principal \"${BHANU}\";
    membership_plans = vec {
      record { variant { Diamond }; 40_000 : nat };
    };
    controllers = vec {
      principal \"bd3sg-teaaa-aaaaa-qaaba-cai\";
      principal \"bd3sg-teaaa-aaaaa-qaaba-cai\";
      principal \"bd3sg-teaaa-aaaaa-qaaba-cai\";
    };
    canister_ids = vec {
    
      record { \"asset_canister\"; principal \"${IC_ASSET_CANISTER}\" };
      record { \"ledger_canister\"; principal \"${LEDGER_CANISTER}\" };
      record { \"post_canister\"; principal \"${POST_CANISTER}\" };
    };
  }
)" --ic


dfx deploy post_canister --argument "(
  record {
    controllers = vec {
      principal \"bd3sg-teaaa-aaaaa-qaaba-cai\";
      principal \"bd3sg-teaaa-aaaaa-qaaba-cai\";
      principal \"bd3sg-teaaa-aaaaa-qaaba-cai\";
    };
    canister_ids = vec {
      record { \"asset_canister\"; principal \"${IC_ASSET_CANISTER}\" };
      record { \"index_canister\"; principal \"${INDEX_CANISTER}\" };

    };
    accounts = vec {
    
 
    };
  },
)" --ic




dfx deploy --ic




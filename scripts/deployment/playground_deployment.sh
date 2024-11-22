
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
dfx canister create asset_handler --playground
dfx canister create post_canister --playground
dfx canister create index_canister --playground

# for compiling canisters
dfx build asset_handler --playground
dfx build post_canister --playground
dfx build index_canister --playground

# Canister IDS
IC_ASSET_CANISTER=$(dfx canister id asset_handler --playground)
INDEX_CANISTER=$(dfx canister id index_canister --playground)
POST_CANISTER=$(dfx canister id post_canister --playground)
FRONTEND_CANISTER=$(dfx canister id post_canister --playground)

# FOR ICP LEDGER
MINTER_ACCOUNT_ID=$(dfx --identity anonymous ledger account-id)
DEFAULT_ACCOUNT_ID=$(dfx --identity default ledger account-id)






# IMP: Review below warnings
# Update code in /src/index_canister/src/lib.rs if below keys are changed 

# dfx deploy index_canister --argument "( record {
#     active_post_canister = principal \"${POST_CANISTER}\";
#     payment_recipient = principal \"${BHANU}\";
#     membership_plans = vec {
#       record { variant { Silver }; 10000 : nat64 };
#       record { variant { Gold }; 20000 : nat64 };
#       record { variant { Platinum }; 40000 : nat64 };
#     };
#     controllers = vec {
#       principal \"bd3sg-teaaa-aaaaa-qaaba-cai\";
#       principal \"bd3sg-teaaa-aaaaa-qaaba-cai\";
#       principal \"bd3sg-teaaa-aaaaa-qaaba-cai\";
#     };
#     canister_ids = vec {
    
#       record { \"asset_canister\"; principal \"${IC_ASSET_CANISTER}\" };
#       record { \"ledger_canister\"; principal \"${IC_ASSET_CANISTER}\" };
#       record { \"post_canister\"; principal \"${POST_CANISTER}\" };
#     };
#   }
# )" --playground

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
)" --playground



# FOR TESTING INIT ARGS OF POST CANISTER
 # record {
  #       username = \"bhanuprata\";
  #       user_id = principal \"fsefm-f46ro-lulwk-ex4sf-z33o5-oihe2-lly2w-uommw-7u5xl-6spjb-eae\";
  #       membership = variant { Guest };
       
  #     };

# dfx deploy --playground




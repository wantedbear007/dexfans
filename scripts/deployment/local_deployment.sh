
set -e

# Controller ids
# dfx identity new Bhanu
# dfx identity use Bhanu
CONTROLLER01=$(dfx identity get-principal --identity default)

# dfx deps init
# dfx deps deploy 
# dfx deps pull
# for creating canisters IDS
dfx canister create icp_ledger_canister
dfx canister create asset_handler
dfx canister create post_canister
dfx canister create index_canister

# for compiling canisters
dfx build icp_ledger_canister
dfx build asset_handler
dfx build post_canister
dfx build index_canister

# Canister IDS
LEDGER_CANISTER=$(dfx canister id icp_ledger_canister)
IC_ASSET_CANISTER=$(dfx canister id asset_handler)
INDEX_CANISTER=$(dfx canister id index_canister)
POST_CANISTER=$(dfx canister id post_canister)
FRONTEND_CANISTER=$(dfx canister id post_canister)

# FOR ICP LEDGER
MINTER_ACCOUNT_ID=$(dfx --identity anonymous ledger account-id)
DEFAULT_ACCOUNT_ID=$(dfx --identity default ledger account-id)



# test canister icp ledger
dfx deploy --specified-id ryjl3-tyaaa-aaaaa-aaaba-cai icp_ledger_canister --argument "
  (variant {
    Init = record {
      minting_account = \"$MINTER_ACCOUNT_ID\";
      initial_values = vec {
        record {
          \"$DEFAULT_ACCOUNT_ID\";
          record {
            e8s = 10_000_000_000 : nat64;
          };
        };
      };
      send_whitelist = vec {};
      transfer_fee = opt record {
        e8s = 10_000 : nat64;
      };
      token_symbol = opt \"LICP\";
      token_name = opt \"Local ICP\";
    }
  })
"



dfx deploy index_canister --argument "( record {
    payment_recipient = principal \"${CONTROLLER01}\";
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
)"


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
)"



# FOR TESTING INIT ARGS OF POST CANISTER
 # record {
  #       username = \"bhanuprata\";
  #       user_id = principal \"fsefm-f46ro-lulwk-ex4sf-z33o5-oihe2-lly2w-uommw-7u5xl-6spjb-eae\";
  #       membership = variant { Guest };
       
  #     };

dfx deploy 




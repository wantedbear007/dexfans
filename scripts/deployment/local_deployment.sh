
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
dfx canister create icp_ledger_canister
# dfx canister create asset_handler
dfx canister create ic_oss_bucket
dfx canister create ic_oss_cluster

dfx canister create post_canister
dfx canister create index_canister
dfx canister create ic_oss_cluster

# for compiling canisters
dfx build icp_ledger_canister
# dfx build asset_handler
dfx build ic_oss_bucket
dfx build ic_oss_cluster
dfx build post_canister
dfx build index_canister
dfx build ic_oss_cluster

# Canister IDS
LEDGER_CANISTER=$(dfx canister id icp_ledger_canister)
IC_ASSET_CANISTER=$(dfx canister id ic_oss_bucket)
INDEX_CANISTER=$(dfx canister id index_canister)
POST_CANISTER=$(dfx canister id post_canister)
IC_OSS_BUCKET_CANISTER=$(dfx canister id ic_oss_bucket)
IC_OSS_CLUSTER=$(dfx canister id ic_oss_cluster)

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

dfx deploy ic_oss_cluster --argument "(opt variant {Init =
  record {
    name = \"LDC Labs\";
    ecdsa_key_name = \"dfx_test_key\";
    schnorr_key_name = \"dfx_test_key\";
    token_expiration = 3600;
    bucket_topup_threshold = 1_000_000_000_000;
    bucket_topup_amount = 5_000_000_000_000;
  }
})"

dfx deploy ic_oss_bucket --argument "(opt variant {Init =
  record {
    name = \"dex Labs\";
    file_id = 0;
    max_file_size = 0;
    max_folder_depth = 10;
    max_children = 10000;
    visibility = 1;
    max_custom_data_size = 4096;
    enable_hash_index = false;
  }
})"





# IMP: Review below warnings
# Update code in /src/index_canister/src/lib.rs if below keys are changed 
dfx deploy index_canister --argument "( record {
    active_post_canister = principal \"${POST_CANISTER}\";
    active_asset_canister = principal \"${IC_ASSET_CANISTER}\";

    payment_recipient = principal \"${BHANU}\";
    membership_plans = vec {
      record { variant { Diamond }; 40_000 : nat };
    };
    controllers = vec {
      principal \"${BHANU}\";
      principal \"bd3sg-teaaa-aaaaa-qaaba-cai\";
      principal \"bd3sg-teaaa-aaaaa-qaaba-cai\";
    };
    canister_ids = vec {
    
      record { \"ledger_canister\"; principal \"${LEDGER_CANISTER}\" };
      record { \"post_canister\"; principal \"${POST_CANISTER}\" };
      record { \"cluster_canister\"; principal \"${IC_OSS_CLUSTER}\" };

    };
  }
)"

      # record { \"asset_canister\"; principal \"${IC_ASSET_CANISTER}\" };


dfx deploy post_canister --argument "(
  record {
    controllers = vec {
      principal \"${INDEX_CANISTER}\";
      principal \"${BHANU}\";
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



dfx deploy 

dfx canister update-settings ic_oss_cluster --add-controller $INDEX_CANISTER
# chmod 777 ./scripts/deployment/add_wasm.sh
# ./scripts/deployment/add_wasm.sh

dfx ledger fabricate-cycles --all --cycles 8000000000000
# dfx canister call ic_oss_cluster admin_create_bucket  
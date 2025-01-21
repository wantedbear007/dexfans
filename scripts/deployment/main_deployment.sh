
set -e

# Controller ids
# dfx identity new Bhanu
# dfx identity use Bhanu
dfx identity new minter --storage-mode=plaintext || true
dfx identity new reciever --storage-mode=plaintext || true


OWNER=$(dfx identity get-principal --identity)

# dfx deps init
# dfx deps deploy 
# dfx deps pull
# for creating canisters IDS
# dfx canister create icp_ledger_canister
dfx canister create ic_oss_bucket --ic
dfx canister create ic_oss_cluster --ic
dfx canister create post_canister --ic
dfx canister create index_canister --ic

# for compiling canisters
# dfx build icp_ledger_canister
dfx build icp_ledger_canister --ic
dfx build ic_oss_bucket --ic
dfx build ic_oss_cluster --ic
dfx build post_canister --ic
dfx build index_canister --ic

# Canister IDS
LEDGER_CANISTER="ryjl3-tyaaa-aaaaa-aaaba-cai"
IC_ASSET_CANISTER=$(dfx canister id ic_oss_bucket --ic)
INDEX_CANISTER=$(dfx canister id index_canister --ic)
POST_CANISTER=$(dfx canister id post_canister --ic)
IC_OSS_BUCKET_CANISTER=$(dfx canister id ic_oss_bucket --ic)
IC_OSS_CLUSTER=$(dfx canister id ic_oss_cluster --ic)


# FOR ICP LEDGER
MINTER_ACCOUNT_ID=$(dfx --identity anonymous ledger account-id)
DEFAULT_ACCOUNT_ID=$(dfx --identity default ledger account-id)


# IMP: Review below warnings
# Update code in /src/index_canister/src/lib.rs if below keys are changed 

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
})" --ic

dfx deploy ic_oss_cluster --argument "(opt variant {Init =
  record {
    name = \"LDC Labs\";
    ecdsa_key_name = \"dfx_test_key\";
    schnorr_key_name = \"dfx_test_key\";
    token_expiration = 3600;
    bucket_topup_threshold = 1_000_000_000_000;
    bucket_topup_amount = 5_000_000_000_000;
  }
})" --ic



dfx deploy index_canister --argument "( record {
    active_post_canister = principal \"${POST_CANISTER}\";
    active_asset_canister = principal \"${IC_ASSET_CANISTER}\";

    payment_recipient = principal \"${OWNER}\";
    membership_plans = vec {
      record { variant { Diamond }; 40_000 : nat };
    };
    controllers = vec {
      principal \"${OWNER}\";
      principal \"bd3sg-teaaa-aaaaa-qaaba-cai\";
      principal \"bd3sg-teaaa-aaaaa-qaaba-cai\";
    };
    canister_ids = vec {
    
      record { \"ledger_canister\"; principal \"${LEDGER_CANISTER}\" };
      record { \"post_canister\"; principal \"${POST_CANISTER}\" };
      record { \"cluster_canister\"; principal \"${IC_OSS_CLUSTER}\" };

    };
  }
)" --ic


dfx deploy post_canister --argument "(
  record {
    controllers = vec {
      principal \"${INDEX_CANISTER}\";
      principal \"${OWNER}\";
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
dfx canister update-settings ic_oss_cluster --add-controller $INDEX_CANISTER --ic




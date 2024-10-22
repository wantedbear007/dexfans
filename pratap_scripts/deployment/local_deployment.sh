
set -e

# Controller ids
# dfx identity new Bhanu
# dfx identity use Bhanu
CONTROLLER01=$(dfx identity get-principal --identity default)

dfx deps init
dfx deps deploy 
dfx deps pull
# for creating canisters IDS
dfx canister create asset_handler
dfx canister create post_canister
dfx canister create dexfans_backend

# for compiling canisters
dfx build asset_handler
dfx build post_canister
dfx build dexfans_backend

# Canister IDS
IC_ASSET_CANISTER=$(dfx canister id asset_handler)
dexfans_backend=$(dfx canister id dexfans_backend)
POST_CANISTER=$(dfx canister id post_canister)




# dfx deploy dexfans_backend --argument "( record {
#       controllers =  vec { principal \"${CONTROLLER01}\" };
#       asset_canister = principal \"${IC_ASSET_CANISTER}\";
#       post_canister = principal \"${POST_CANSTER}\";
#       icp_ledger_canister = principal \"${POST_CANSTER}\";
#   }
# )"


# dfx deploy dexfans_backend --argument "( record {
#     icp_ledger_canister = principal "bd3sg-teaaa-aaaaa-qaaba-cai";
#     controllers = vec {
#       principal "bd3sg-teaaa-aaaaa-qaaba-cai";
#       principal "bd3sg-teaaa-aaaaa-qaaba-cai";
#       principal "bd3sg-teaaa-aaaaa-qaaba-cai";
#     };
#     canister_ids = vec {
#       record { "a"; principal "bd3sg-teaaa-aaaaa-qaaba-cai" };
#       record { "b"; principal "bd3sg-teaaa-aaaaa-qaaba-cai" };
#     };
#     asset_canister = principal "bd3sg-teaaa-aaaaa-qaaba-cai";
#     post_canister = principal "bd3sg-teaaa-aaaaa-qaaba-cai";
#   }
# )"

dfx deploy dexfans_backend --argument "( record {
    controllers = vec {
      principal \"bd3sg-teaaa-aaaaa-qaaba-cai\";
      principal \"bd3sg-teaaa-aaaaa-qaaba-cai\";
      principal \"bd3sg-teaaa-aaaaa-qaaba-cai\";
    };
    canister_ids = vec {
      record { \"asset_canister\"; principal \"${IC_ASSET_CANISTER}\" };
      record { \"ledger_canister\"; principal \"${IC_ASSET_CANISTER}\" };
      record { \"post_canister\"; principal \"${POST_CANISTER}\" };
    };
  }
)"



# deploy post canisters 
dfx deploy post_canister --argument "( record {
      asset_canister =  vec { principal \"${IC_ASSET_CANISTER}\" };
      parent_canister = principal \"${dexfans_backend}\";
  }
)"

dfx deploy 

# (vec { record { "hello"; "world" } })
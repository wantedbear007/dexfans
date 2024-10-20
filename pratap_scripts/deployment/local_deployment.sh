
set -e

# for creating canisters IDS
# dfx canister create ic_asset_handler
dfx canister create post_canister
dfx canister create dexfans_backend

# for compiling canisters
# dfx build ic_asset_handler
dfx build post_canister
dfx build dexfans_backend

# Canister IDS
IC_ASSET_CANISTER=$(dfx canister id dexfans_backend)
dexfans_backend=$(dfx canister id dexfans_backend)

# deploy post canisters 
dfx deploy post_canister --argument "( record {
      asset_canister =  vec { principal \"${IC_ASSET_CANISTER}\" };
      parent_canister = principal \"${dexfans_backend}\";
  }
)"

dfx deploy 
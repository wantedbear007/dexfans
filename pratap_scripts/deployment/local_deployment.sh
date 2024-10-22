
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




dfx deploy dexfans_backend --argument "( record {
      controllers =  vec { principal \"${CONTROLLER01}\" };
      asset_canister = principal \"${IC_ASSET_CANISTER}\";
  }
)"

# deploy post canisters 
dfx deploy post_canister --argument "( record {
      asset_canister =  vec { principal \"${IC_ASSET_CANISTER}\" };
      parent_canister = principal \"${dexfans_backend}\";
  }
)"

dfx deploy 
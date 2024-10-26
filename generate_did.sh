cargo build --release --target wasm32-unknown-unknown --package index_canister

candid-extractor target/wasm32-unknown-unknown/release/index_canister.wasm >src/index_canister/index_canister.did

cargo build --release --target wasm32-unknown-unknown --package post_canister

candid-extractor target/wasm32-unknown-unknown/release/post_canister.wasm >src/post_canister/post_canister.did

# cargo build --release --target wasm32-unknown-unknown --package ic_asset_handler

# candid-extractor target/wasm32-unknown-unknown/release/ic_asset_handler.wasm >src/ic_asset_handler/ic_asset_handler.did


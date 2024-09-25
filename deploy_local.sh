
# To generate associated functions in did file
cargo build --release --target wasm32-unknown-unknown --package dexfans_backend

candid-extractor target/wasm32-unknown-unknown/release/dexfans_backend.wasm >src/dexfans_backend/dexfans_backend.did


# deploy 
dfx deploy dexfans_backend
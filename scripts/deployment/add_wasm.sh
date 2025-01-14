# #!/bin/bash

# WASM_PATH=".dfx/local/canisters/ic_oss_bucket/ic_oss_bucket.wasm"

# if [[ ! -f "$WASM_PATH" ]]; then
#     echo "Error: WASM file not found at $WASM_PATH"
#     exit 1
# fi

# DESCRIPTION="jan24"


# BASE64_WASM=$(base64 "$WASM_PATH")

# ARGUMENT="(record { description = \"$DESCRIPTION\"; wasm = blob \"$BASE64_WASM\" }, null)"


# echo "Adding ic_oss_bucket canister wasm to ic_oss_cluster..."
# dfx canister call ic_oss_cluster admin_add_wasm "$ARGUMENT"

# if [[ $? -eq 0 ]]; then
#     echo "WASM module added successfully."
# else
#     echo "Error: Failed to add the WASM module."
#     exit 2
# fi

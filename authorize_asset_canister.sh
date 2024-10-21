

# http://br5f7-7uaaa-aaaaa-qaaca-cai.localhost:4943/f/1


export MYID=$(dfx identity get-principal)
dfx canister call asset_handler admin_set_managers "(vec {principal \"$MYID\"; principal \"pxfqr-x3orr-z5yip-7yzdd-hyxgd-dktgh-3awsk-ohzma-lfjzi-753j7-tae\"})"


# For enabling link access
dfx canister call asset_handler admin_update_bucket "(
  record {
    status = null;
    trusted_eddsa_pub_keys = null;
    name = null;
    max_custom_data_size = null;
    max_children = null;
    enable_hash_index = null;
    max_file_size = null;
    visibility = opt (1 : nat8);
    max_folder_depth = null;
    trusted_ecdsa_pub_keys = null;
  },
)"

# to access file
# http://ASSET_HANDLER_CANISTER_ID.localhost:4943/f/FILE_ID

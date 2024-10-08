use sha2::Digest;

// function to get uuid
pub async fn commons_get_uuid() -> String {
    format!(
        "{:x}",
        sha2::Sha256::digest(
            &ic_cdk::api::management_canister::main::raw_rand()
                .await
                .unwrap()
                .0
        )
    )
}



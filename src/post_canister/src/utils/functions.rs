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

#[ic_cdk::query]
pub fn get_canister_meta_data() -> Result<crate::models::types::CanisterMetaData, String> {
    crate::with_read_state(|state| match state.canister_meta_data.get(&0) {
        Some(val) => Ok(val),
        None => return Err(String::from(dexfans_types::constants::ERROR_FAILED_CANISTER_DATA)),
    })
}

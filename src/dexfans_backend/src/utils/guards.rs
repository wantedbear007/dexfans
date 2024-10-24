// to prevent anonymous calls
pub fn guard_prevent_anonymous() -> Result<(), String> {
    if ic_cdk::api::caller() == candid::Principal::anonymous() {
        return Err(String::from(
            dexfans_types::constants::WARNING_ANONYMOUS_CALL,
        ));
    }

    Ok(())
}

// controllers accesss
pub fn guard_only_admin() -> Result<(), String> {
    guard_prevent_anonymous()?;

    crate::with_read_state(|state| match state.canister_meta_data.get(&0) {
        Some(_val) => Ok(()),
        None => return Err(String::from(dexfans_types::constants::WARNING_ADMIN_ONLY)),
    })
}

// post canister only
pub fn guard_post_canister_exclusive() -> Result<(), String> {
    guard_prevent_anonymous()?;

    crate::with_read_state(|state| match state.canister_meta_data.get(&0) {
        Some(val) => {
            for canister in val.all_post_canisters.iter() {
                if canister == &ic_cdk::api::caller() {
                    return Ok(());
                }
            }
            return Err(String::from(dexfans_types::constants::ERROR_UNAUTHORIZED));
        }
        None => {
            return Err(String::from(
                dexfans_types::constants::ERROR_FAILED_CANISTER_DATA,
            ))
        }
    })
}

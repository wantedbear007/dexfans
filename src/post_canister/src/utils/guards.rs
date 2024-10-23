use crate::with_read_state;

// to prevent anonymous calls
pub fn guard_prevent_anonymous() -> Result<(), String> {
    if ic_cdk::api::caller() == candid::Principal::anonymous() {
        return Err(String::from(
            dexfans_types::constants::WARNING_ANONYMOUS_CALL,
        ));
    }

    Ok(())
}

// to allow parnet canister only (for intercanister calls)
pub fn guard_parent_canister_only() -> Result<(), String> {
    guard_prevent_anonymous()?;
    with_read_state(|state| match state.canister_meta_data.get(&0) {
        Some(val) => {
            if val.parent_canister == ic_cdk::api::caller() {
                return Ok(());
            } else {
                return Err(String::from(dexfans_types::constants::WARNING_ADMIN_ONLY));
            };
        }
        None => {
            return Err(String::from(
                dexfans_types::constants::ERROR_FAILED_CANISTER_DATA,
            ))
        }
    })
}



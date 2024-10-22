// to prevent anonymous calls
pub fn guard_prevent_anonymous() -> Result<(), String> {
    if ic_cdk::api::caller() == candid::Principal::anonymous() {
        return Err(String::from(
            dexfans_types::constants::WARNING_ANONYMOUS_CALL,
        ));
    }

    Ok(())
}

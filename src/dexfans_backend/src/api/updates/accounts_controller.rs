use crate::with_write_state;

pub fn controller_create_account(
    args: crate::models::user_types::UserProfile,
) -> Result<(), String> {
    with_write_state(|state| {

      // checking if user already exists
        if state.account.contains_key(&ic_cdk::api::caller()) {
            return Err(String::from(
                crate::utils::constants::WARNING_ACCOUNT_EXISTS,
            ));
        } else {
        }
        state.account.insert(ic_cdk::api::caller(), args);
        Ok(())
    })
}

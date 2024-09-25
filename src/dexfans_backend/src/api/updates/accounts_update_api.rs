use crate::utils::guards::*;

#[ic_cdk::update(guard=guard_prevent_anonymous)]
pub fn api_create_account(args: crate::models::user_types::UserProfile) -> Result<String, String> {
    super::accounts_controller::controller_create_account(args).map_err(|err| {
        format!(
            "{}{}",
            crate::utils::constants::ERROR_ACCOUNT_ERROR,
            err.to_string()
        )
    })?;

    Ok(String::from(
        crate::utils::constants::SUCCESS_ACCOUNT_CREATED,
    ))
}

use validator::Validate;

// to validate input args
pub fn input_validator<T>(args: &T) -> super::types::Response
where
    T: Validate,
{
    args.validate()
        .map_err(|err| format!("Validation Error: {}", err))
}

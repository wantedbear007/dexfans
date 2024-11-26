// app essentials
pub const ESSENTIALS_APP_NAME: &str = "dexfans !";
// CANISTER ID KEYs
pub const ESSENTIAL_POST_CANISTER_ID_CODE: u8 = 1;
pub const ESSENTIAL_ASSET_CANISTER_ID_CODE: u8 = 2;
pub const ESSENTIAL_LEDGER_CANISTER_ID_CODE: u8 = 3;
pub const ESSENTIAL_FRONTEND_CANISTER_ID_CODE: u8 = 4;
pub const ESSENTIAL_NOTIFICATION_EXPIRING: u64 = 24 * 60 * 60 * 1_000_000_000; // 1 day
pub const ESSENTIAL_POST_PARENT_CANISTER: &str = "index_canister";
pub const ESSENTIAL_MEMBERSHIP_VALIDITY: u64 = 30 * 24 * 60 * 60 * 1_000_000_000; // 30 days
pub const ESSENTIAL_POST_CANISTER_CYCLE_THRESHOLD: u128 = 1_000_000_000_000; // 1T Cycles
pub const ESSENTIAL_SUGGESTED_USER_THRESHOLD: u32 = 15;
pub const ESSENTIAL_FUZZY_SEARCH_THRESHOLD: usize = 3;
pub const ESSENTIAL_IMAGE_COUNT_LIMIT: usize = 4;
pub const ESSENTIAL_CAPTCHA_THRESHOLD: usize = 50;
pub const ESSENTIAL_CAPTCHA_HARDNESS: u8 = 32; // higher the hardness lower the performance (should not exceed 255)
pub const ESSENTIAL_CAPTCHA_LIFETIME: super::types::Milliseconds = 3 * MINUTE_IN_MS; // 5 minute
// TIME
pub const SECOND_IN_MS: super::types::Milliseconds = 1000;
pub const MINUTE_IN_MS: super::types::Milliseconds = SECOND_IN_MS * 60;
// function names
pub const FUNCTION_GET_POST_PRICE: &str = "ic_get_price";
pub const FUNCTION_GET_MEDIA_PRICE: &str = "ic_get_media_price";

// WARNINGS
pub const WARNING_ANONYMOUS_CALL: &str = "Anonymous calls are not permitted!";
pub const WARNING_IMAGES_LIMIT: &str = "Excess images found";
pub const WARNING_ACCOUNT_EXISTS: &str = "This Principal ID is already registered on the platform";
pub const WARNING_ADMIN_ONLY: &str = "Admin privileges are required";
pub const WARNING_CONTROLLER_EXIST: &str = "Principal is already assigned as a controller";
pub const WARNING_SAME_VALUE: &str = "User already possesses this membership level";
pub const WARNING_ALERADY_EXIST: &str = "User Already exist";
pub const WARNING_SAME_MEMBERSHIP: &str = "You are already a member of this category";
pub const WARNING_HIGHER_MEMBERSHIP: &str =
    "Higher category of membership found, try after current membership expires";
pub const WARNING_ALREADY_PURCHASED: &str = "Post is already purchased";
pub const WARNING_POST_IS_FREE: &str = "Post is already free";
pub const WARNING_POST_OWNER: &str = "You owns the post";
pub const WARNING_CAPTCHA_REACHED: &str = "Wait for some time captcha threshold reached";

// Erros
pub const ERROR_ACCOUNT_ERROR: &str = "Accounts Error: ";
pub const ERROR_ACCOUNT_NOT_REGISTERED: &str = "Principal id is not registered with the platform";
pub const ERROR_CANISTER_ID: &str = "canister not found";
pub const ERROR_FAILED_CANISTER_DATA: &str = "Failed to get canister meta data";
pub const ERROR_FAILED_CALL: &str = "Failed to perform operation, try again later";
pub const ERROR_ENCODE_FAILED: &str = "Failed to encode data";
pub const ERROR_DECODE_FAILED: &str = "Failed to decode data";
pub const ERROR_FAILED_INTER_CANISTER: &str = "Failed to perform intercanister call";
pub const ERROR_PROFILE_UPDATE: &str = "Failed to update profile, try again";
pub const ERROR_POST_NOT_EXIST: &str = "No post associated with provided id";
pub const ERROR_UNAUTHORIZED: &str = "Unauthorized call";
pub const ERROR_PAYMENT_FAILED: &str = "ICP payment failed, try again or contact admin";
pub const ERROR_INVALID_CANISTER: &str = "Unrecognized canister.";

// SUCCESS
pub const SUCCESS_ACCOUNT_CREATED: &str = "Account successfully created";
pub const SUCCESS_POST_CREATED: &str = "Post successfully created";
pub const SUCESSS_POST_UPDATED: &str = "Post updated successfully";
pub const SUCCESS_POST_DELETED: &str = "Post deleted successfully";
pub const SUCCESS_POST_SAVED: &str = "Post saved successfully";
pub const SUCCESS_POST_ARCHIVED: &str = "Post archived successfully";

// storable value sizes
pub const STORABLE_USER_MAX_VALUE_SIZE: u32 = 600;

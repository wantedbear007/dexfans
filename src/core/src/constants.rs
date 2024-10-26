// app essentials
pub const ESSENTIALS_APP_NAME: &str = "dexfans !";
// CANISTER ID KEYs
pub const ESSENTIAL_POST_CANISTER_ID_CODE: u8 = 1;
pub const ESSENTIAL_ASSET_CANISTER_ID_CODE: u8 = 2;
pub const ESSENTIAL_LEDGER_CANISTER_ID_CODE: u8 = 3;
pub const ESSENTIAL_FRONTEND_CANISTER_ID_CODE: u8 = 4;
pub const ESSENTIAL_NOTIFICATION_EXPIRING: u64 = 24 * 60 * 60 * 1_000_000_000; // 1 day
pub const ESSENTIAL_POST_PARENT_CANISTER: &str = "parent_id";
pub const ESSENTIAL_MEMBERSHIP_VALIDITY: u64 = 30 * 24 * 60 * 60 * 1_000_000_000; // 30 days

// WARNINGS
pub const WARNING_ANONYMOUS_CALL: &str = "Anonymous principal not allowed !";
pub const WARNING_ACCOUNT_EXISTS: &str = "Principal ID is already registered with the platform";
pub const WARNING_ADMIN_ONLY: &str = "Admin access required";
pub const WARNING_CONTROLLER_EXIST: &str = "Principal is already present";
pub const WARNING_SAME_VALUE: &str = "User already have this membership";
pub const WARNING_ALERADY_EXIST: &str = "Already exist";
pub const WARNING_SAME_MEMBERSHIP: &str = "You are already a member of this category";
pub const WARNING_HIGHER_MEMBERSHIP: &str = "Higher category of membership found, try after current membership expires";

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

// SUCCESS
pub const SUCCESS_ACCOUNT_CREATED: &str = "Account successfully created";
pub const SUCCESS_POST_CREATED: &str = "Post successfully created";
pub const SUCESSS_POST_UPDATED: &str = "Post updated successfully";
pub const SUCCESS_POST_DELETED: &str = "Post deleted successfully";

// storable value sizes
pub const STORABLE_USER_MAX_VALUE_SIZE: u32 = 600;

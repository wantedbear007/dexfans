use crate::utils::guards::*;
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, CandidType)]
pub struct Challange {
    pub base64_img: String,
}

#[ic_cdk::update(guard = guard_prevent_anonymous)]
async fn api_create_captcha() -> Result<Challange, String> {
    // Check if the user is already registered
    if crate::with_read_state(|state| state.account.contains_key(&ic_cdk::api::caller())) {
        return Err(core::constants::WARNING_ACCOUNT_EXISTS.into());
    }

    match super::init::captcha_write_state(|state| {
        if let Some(mut cap) = state.captchas.get(&0) {
            if cap.all.len() >= core::constants::ESSENTIAL_CAPTCHA_THRESHOLD {
                return Err(String::from(core::constants::WARNING_CAPTCHA_REACHED));
            }

            // Retain only non-expired captchas
            captcha_filter(&mut cap, false, true);

            state.captchas.insert(0, cap.clone());

            for x in cap.all.iter() {
                if x.created_by == ic_cdk::api::caller() {
                    return Err(String::from(core::constants::WARNING_CAPTCHA_REACHED));
                }
            }

            Ok(cap)
        } else {
            Err(core::constants::ERROR_FAILED_CANISTER_DATA.into())
        }
    }) {
        Ok(mut val) => {
            let builder = ic_captcha::CaptchaBuilder::new();

            let (seed,) = ic_cdk::api::management_canister::main::raw_rand()
                .await
                .unwrap();

            let captcha = builder.generate(&seed, None);

            let sol = crate::models::types::CaptchaSolution {
                created_at: ic_cdk::api::time(),
                data: captcha.text(),
                created_by: ic_cdk::api::caller(),
            };

            val.all.push(sol);

            super::init::captcha_write_state(|state| {
                if let Some(_) = state.captchas.get(&0) {
                    state.captchas.insert(0, val);

                    // debug capsol
                    ic_cdk::println!("cap sol: {}", captcha.text());

                    Ok(Challange {
                        base64_img: captcha.to_base64(0),
                    })
                } else {
                    Err(String::from(core::constants::ERROR_FAILED_CANISTER_DATA))
                }
            })
        }
        Err(err) => Err(err),
    }
}

// to verify captch
pub(crate) fn verify_captcha(id: candid::Principal, text: &String) -> core::types::Response {
    super::init::CAPTCHA_STATE.with_borrow_mut(|state| {
        if let Some(mut val) = state.captchas.get(&0) {
            // remove all expired captchas
            captcha_filter(&mut val, false, true);

            for cap in val.all.iter() {
                if cap.created_by == id && &cap.data == text {
                    state.captchas.insert(0, val);
                    return Ok(());
                }
            }

            state.captchas.insert(0, val);

            Err(String::from(core::constants::ERROR_INVALID_CAPTCHA))
        } else {
            Err(String::from(core::constants::ERROR_INVALID_CAPTCHA))
        }
    })
}

// #[ic_cdk::query]
// fn debug_get_all_captchas() -> Vec<crate::models::types::CaptchaSolution> {
//     super::init::captcha_write_state(|state| match state.captchas.get(&0) {
//         Some(cap) => {
//             // captcha_filter(&mut cap, false);
//             // state.captchas.insert(0, cap.clone());

//             cap.all
//         }
//         None => Vec::new(),
//     })
// }

// filter captcha for time and user
fn captcha_filter(args: &mut crate::models::types::Captchas, usr_based: bool, time_based: bool) {
    if time_based {
        args.all.retain(|val| {
            val.created_at + core::constants::ESSENTIAL_CAPTCHA_LIFETIME > ic_cdk::api::time()
        });
    }
    if usr_based {
        args.all.retain(|val| {
            // Retain elements where the condition is true
            val.created_by != ic_cdk::api::caller()
        });
    }

    // ic_cdk::println!("va {:?}", args.all);
}

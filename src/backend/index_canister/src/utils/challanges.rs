use std::borrow::BorrowMut;

use candid::CandidType;
use ic_cdk::caller;
use rand::{Rng, RngCore};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct CaptchaChallanges1 {
    pub all_captcha: std::collections::HashMap<candid::Principal, CaptchaSolution1>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct CaptchaSolution1 {
    pub created: core::types::TimestampMillis,
    pub data: String,
}

#[derive(Serialize, Deserialize, CandidType)]
pub struct Challange {
    // pub key: core::types::CaptchaKey,
    // pub key: candid::Principal,
    pub base64_img: String,
}

// // impl CaptchaChallanges1 {
//     pub(crate) async fn create(
//         &mut self,
//         now: core::types::TimestampMillis,
//     ) -> Result<Challange, String> {
//         // prevent from unlimited captcha creation
//         if self.all_captcha.len() >= core::constants::ESSENTIAL_CAPTCHA_THRESHOLD {
//             return Err(String::from(core::constants::WARNING_CAPTCHA_REACHED));
//         }

//         // delete all expired captchas
//         self.all_captcha.retain(|_, f| !f.expired(now));

//         // add validations
//         let builder = ic_captcha::CaptchaBuilder::new();

//         let (seed,) = ic_cdk::api::management_canister::main::raw_rand()
//             .await
//             .unwrap();

//         let seed: &[u8] = &seed;

//         let captcha = builder.generate(seed, None);

//         self.all_captcha.insert(
//             ic_cdk::api::caller(),
//             CaptchaSolution1 {
//                 created: now,
//                 data: captcha.text(),
//             },
//         );

//         ic_cdk::println!("{:?}", self.all_captcha);

//         ic_cdk::println!("data is: {}", captcha.text());

//         Ok(Challange {
//             base64_img: captcha.to_base64(0),
//             key: ic_cdk::api::caller(),
//         })
//     }

//     pub fn verify(&mut self, content: String, now: core::types::TimestampMillis) -> bool {
//         // remove all expired captchas
//         self.all_captcha.retain(|_, f| !f.expired(now));

//         for (_, cap) in self.all_captcha.iter() {
//             if cap.data == content {
//                 return true;
//                 // todo delete if captcha is verified !
//             }
//         }

//         false
//     }

//     pub fn get_all_cap(self) -> Vec<CaptchaSolution1> {
//         let mut data: Vec<CaptchaSolution1> = Vec::new();
//         for x in self.all_captcha {
//             data.push(x.1);
//         }

//         data
//     }
// }

impl crate::models::types::CaptchaSolution {
    pub fn expired(&self, now: core::types::Milliseconds) -> bool {
        now > core::constants::ESSENTIAL_CAPTCHA_LIFETIME
    }
}

// #[ic_cdk::update]
// async fn bhanu() -> Result<Challange, String> {
//     let mut x = CaptchaChallanges1::default();
//     x.create(ic_cdk::api::time()).await
// }

// #[ic_cdk::query]
// fn bhanuverfiy(text: String) -> bool {
//     let mut x = CaptchaChallanges1::default();
//     x.verify(text, ic_cdk::api::time())
// }

// #[ic_cdk::query]
// fn bhanucap() -> Vec<CaptchaSolution1> {
//     let mut x = CaptchaChallanges1::default();
//     x.get_all_cap()
// }

#[ic_cdk::update]
async fn api_create_captcha() -> Result<Challange, String> {
    // Check if the user is already registered
    if crate::with_read_state(|state| state.account.contains_key(&ic_cdk::api::caller())) {
        return Err(core::constants::WARNING_ACCOUNT_EXISTS.into());
    }

    match super::init::CAPTCHA_STATE.with_borrow_mut(|state| {
        let now = ic_cdk::api::time();

        if let Some(mut cap) = state.captchas.get(&0) {
            if cap.all.len() >= core::constants::ESSENTIAL_CAPTCHA_THRESHOLD {
                return Err(String::from(core::constants::WARNING_CAPTCHA_REACHED));
            }

            // Retain only non-expired captchas
            cap.all
                .retain(|f| !f.expired(now) && f.created_by != ic_cdk::api::caller());

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
                created: ic_cdk::api::time(),
                data: captcha.text(),
                created_by: ic_cdk::api::caller(),
            };

            // for debug
            ic_cdk::println!("{}", captcha.text());

            val.all.push(sol);

            super::init::CAPTCHA_STATE.with_borrow_mut(|state| {
                if let Some(_) = state.captchas.get(&0) {
                    state.captchas.insert(0, val);

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

#[ic_cdk::query]
// to verify captch
fn verify_captcha(id: candid::Principal, text: String) -> bool {
    super::init::CAPTCHA_STATE.with_borrow_mut(|state| {
        if let Some(mut val) = state.captchas.get(&0) {
            // remove all expired captchas
            val.all.retain(|f| f.expired(ic_cdk::api::time()));

            for cap in val.all.iter() {
                if cap.created_by == id && cap.data == text {
                    val.all.retain(|f| f.created_by != id);
                    state.captchas.insert(0, val);
                    return true;
                }
            }

            state.captchas.insert(0, val);
            false
        } else {
            false
        }
    })
}

#[ic_cdk::query]
fn ver_get_all_captcha() -> Vec<crate::models::types::CaptchaSolution> {
    super::init::CAPTCHA_STATE.with_borrow_mut(|state| {
        if let Some(mut val) = state.captchas.get(&0) {
            // remove all expired captchas
           val.all
        } else {
            Vec::new()
        }
    })
}

// impl CaptchaChallanges {
//     pub fn create_captcha<R: RngCore>(
//         &mut self,
//         current_time: core::types::TimestampMillis,
//         mut rng: R,
//     ) -> Option<Challange> {
//         // deleting all expired challanges
//         self.all_captcha.retain(|_, s| !s.expired(current_time));

//         // add inflight check  canisters/identity/impl/src/model/challenges.rs
//         const MAX_TRIES: u8 = 10;
//         for _ in 0..MAX_TRIES {
//             let key = rng.next_u32();

//             if let std::collections::hash_map::Entry::Vacant(e) = self.all_captcha.entry(key) {
//                 let builder = ic_captcha::CaptchaBuilder::new();
//                 let seed: [u8; 32] = rng.gen();
//                 let captcha = builder.generate(&seed, None);

//                 // Remember the solution
//                 e.insert(CaptchaSolution1 {
//                     created: current_time,
//                     data: captcha.text(),
//                 });

//                 // Return the challenge
//                 return Some(Challange {
//                     base64_img: captcha.to_base64(0),
//                     key,
//                 });
//             }
//         }
//         ic_cdk::trap(&format!(
//             "Could not find a new challenge key after {MAX_TRIES} tries"
//         ));
//     }
// }

// impl CaptchaSolution {
//     pub fn expired(&self, now: core::types::TimestampMillis) -> bool {
//         now > core::constants::ESSENTIAL_CAPTCHA_LIFETIME
//     }
// }

// #[ic_cdk::update]
// async fn get() -> Option<Challange> {

//   let builder = ic_captcha::CaptchaBuilder::new();

//   let lol: &str = "abc";

//   // let captcha = builder.generate(b"random seed 0", None);
//   let captcha = builder.generate(b"bhanu", None);

//   ic_cdk::println!("text: {}", captcha.text());
//   println!("base_img: {}", captcha.to_base64(0));

//   // gen random number
//   let random = crate::utils::init::RNG.with(|rng| {
//     let mut rng = rng.borrow_mut();
//     let rng = rng.as_mut().expect("RNG not initialized"); // Ensure RNG is initialized
//     rng.gen_range(0,12) // Generate a number in the specified range
// });

//   Some(Challange {
//     base64_img: captcha.to_base64(0),
//     key: random
//   })

//   // Challange::new

//     // let mut challange = CaptchaChallanges::default();

//     // let rng = rand::thread_rng();

//     // if let Some(challange) = challange.create_captcha(ic_cdk::api::time(), rng) {
//     //     println!("Captcha Key: {}", challange.key);
//     //     println!("Captcha Image (base64): {}", challange.base64_img);

//     //     return Some(Challange {
//     //         base64_img: challange.base64_img,
//     //         key: challange.key,
//     //     })
//     // } else {
//     //     println!("Failed to create captcha.");
//     //     ic_cdk::trap("dsfs")
//     // }
// }

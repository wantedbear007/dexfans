use rand::{Rng, RngCore};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct CaptchaChallanges {
    pub all_captcha: std::collections::HashMap<core::types::CaptchaKey, CaptchaSolution>,
}

#[derive(Serialize, Deserialize)]
pub struct CaptchaSolution {
    pub created: core::types::TimestampMillis,
    pub data: String,
}

#[derive(Serialize, Deserialize)]
pub struct Challange {
    pub key: core::types::CaptchaKey,
    pub base64_img: String,
}

impl CaptchaChallanges {
    pub fn create_captcha<R: RngCore>(
        &mut self,
        current_time: core::types::TimestampMillis,
        mut rng: R,
    ) -> Option<Challange> {
        // deleting all expired challanges
        self.all_captcha.retain(|_, s| !s.expired(current_time));

        // add inflight check  canisters/identity/impl/src/model/challenges.rs
        const MAX_TRIES: u8 = 10;
        for _ in 0..MAX_TRIES {
            let key = rng.next_u32();

            if let std::collections::hash_map::Entry::Vacant(e) = self.all_captcha.entry(key) {
                let builder = ic_captcha::CaptchaBuilder::new();
                let seed: [u8; 32] = rng.gen();
                let captcha = builder.generate(&seed, None);

                // Remember the solution
                e.insert(CaptchaSolution {
                    created: current_time,
                    data: captcha.text(),
                });

                // Return the challenge
                return Some(Challange {
                    base64_img: captcha.to_base64(0),
                    key,
                });
            }
        }
        ic_cdk::trap(&format!(
            "Could not find a new challenge key after {MAX_TRIES} tries"
        ));
    }
}

impl CaptchaSolution {
    pub fn expired(&self, now: core::types::TimestampMillis) -> bool {
        now > core::constants::ESSENTIAL_CAPTCHA_LIFETIME
    }
}

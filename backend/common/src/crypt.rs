use data_encoding::HEXUPPER;
use ring::{
    error::Unspecified,
    pbkdf2,
    rand::{self, SecureRandom},
};
use std::num::NonZeroU32;

const SALT_LEN: usize = 16;
const HASH_LEN: usize = 32;
static HASH_ALGO: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;

#[derive(Clone)]
pub struct Crypt {
    rng: rand::SystemRandom,
    iterations: NonZeroU32,
}

impl Crypt {
    pub fn new() -> Self {
        Crypt {
            rng: rand::SystemRandom::new(),
            iterations: NonZeroU32::new(100_000).unwrap(),
        }
    }

    pub fn generate_password_hash(
        self,
        password: &String,
    ) -> Result<(String, String), Unspecified> {
        let mut salt = [0u8; SALT_LEN];
        self.rng.fill(&mut salt)?;

        let mut pbkdf2_hash = [0u8; HASH_LEN];
        pbkdf2::derive(
            HASH_ALGO,
            self.iterations,
            &salt,
            password.as_bytes(),
            &mut pbkdf2_hash,
        );

        Ok((HEXUPPER.encode(&pbkdf2_hash), HEXUPPER.encode(&salt)))
    }

    pub fn verify_password(
        self,
        password: &String,
        password_hash: &String,
        password_salt: &String,
    ) -> bool {
        let decoded_hash = HEXUPPER.decode(password_hash.as_bytes()).unwrap();
        let decoded_salt = HEXUPPER.decode(password_salt.as_bytes()).unwrap();
        match pbkdf2::verify(
            HASH_ALGO,
            self.iterations,
            &decoded_salt,
            password.as_bytes(),
            &decoded_hash,
        ) {
            Ok(()) => true,
            Err(_) => false,
        }
    }
}

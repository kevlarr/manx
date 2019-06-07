use argonautica::{Hasher, Verifier};
use rand::{self, Rng};

use crate::ApiResult;

const KEY_LENGTH: u8 = 6;
const KEY_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                         abcdefghijklmnopqrstuvwxyz\
                         0123456789_-";

pub fn hash(secret: &str, password: &str) -> ApiResult<String> {
    Ok(Hasher::default()
        .with_secret_key(secret)
        .with_password(password)
        .hash()?)
}

pub fn verify(secret: &str, hash: &str, password: &str) -> ApiResult<bool> {
    Ok(Verifier::default()
        .with_secret_key(secret)
        .with_hash(hash)
        .with_password(password)
        .verify()?)
}

// See: https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
pub fn rand_key() -> String {
    let mut rng = rand::thread_rng();

    (0..KEY_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0, KEY_CHARSET.len());
            char::from(KEY_CHARSET[idx])
        })
        .collect()
}

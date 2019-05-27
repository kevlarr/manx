use argonautica::{Hasher, Verifier};

use crate::ApiResult;
use crate::error::ApiError;

pub fn hash(secret: &str, password: &str) -> ApiResult<String> {
    Hasher::default()
        .with_secret_key(secret)
        .with_password(password)
        .hash()
        .map_err(ApiError::from)
}

pub fn verify(secret: &str, hash: &str, password: &str) -> ApiResult<bool> {
    Verifier::default()
        .with_secret_key(secret)
        .with_hash(hash)
        .with_password(password)
        .verify()
        .map_err(ApiError::from)
}

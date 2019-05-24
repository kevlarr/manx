mod error;

use argonautica::{Hasher, Verifier};
pub use error::EncryptionError;

type EncryptionResult<T> = std::result::Result<T, EncryptionError>;

pub fn hash(secret: &str, password: &str) -> EncryptionResult<String> {
    Hasher::default()
        .with_secret_key(secret)
        .with_password(password)
        .hash()
        .map_err(|e| EncryptionError::HashError(format!("{}", e)))
}

pub fn verify(secret: &str, hash: &str, password: &str) -> EncryptionResult<bool> {
    Verifier::default()
        .with_secret_key(secret)
        .with_hash(hash)
        .with_password(password)
        .verify()
        .map_err(|e| EncryptionError::VerifyError(format!("{}", e)))
}

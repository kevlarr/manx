use argonautica::{Hasher, Verifier};
use std::{error, fmt};

#[derive(Clone, Debug)]
pub enum EncryptionError {
    HashError(String),
    VerifyError(String),
}

impl fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use EncryptionError::*;

        match self {
            HashError(password) => write!(f, "Unable to hash password '{}'", password),
            VerifyError(msg) => write!(f, "Unable to verify password: {}", msg),
        }
    }
}

impl error::Error for EncryptionError {
    fn description(&self) -> &str {
        use EncryptionError::*;

        match self {
            HashError(_) => "Unable to hash password",
            VerifyError(_) => "Unable to verify password",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

type Result<T> = std::result::Result<T, EncryptionError>;

pub fn hash(secret: &str, password: &str) -> Result<String> {
    Hasher::default()
        .with_secret_key(secret)
        .with_password(password)
        .hash()
        .map_err(|_| EncryptionError::HashError(password.to_string()))
}

pub fn verify(secret: &str, hash: &str, password: &str) -> Result<bool> {
    Verifier::default()
        .with_secret_key(secret)
        .with_hash(hash)
        .with_password(password)
        .verify()
        .map_err(|e| EncryptionError::VerifyError(format!("{}", e)))
}

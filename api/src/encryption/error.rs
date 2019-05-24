use std::{error, fmt};

#[derive(Clone, Debug)]
pub enum EncryptionError {
    HashError(String),
    VerifyError(String),
}

impl fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use EncryptionError::*;

        let mut write = |verb, msg| write!(f, "Unable to {}: {}", verb, msg);

        match self {
            HashError(msg) => write("hash", msg),
            VerifyError(msg) => write("verify", msg),
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

mod error;
pub mod organizations;
pub mod organization_users;
pub mod users;

pub use error::StoreError;

pub type StoreResult<T> = std::result::Result<T, StoreError>;

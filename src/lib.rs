pub mod accounts;
pub mod client;
pub mod error;
pub mod setting;

pub type Result<T> = std::result::Result<T, error::Error>;

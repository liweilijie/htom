pub mod error;
pub mod retrieve;

type Result<T> = std::result::Result<T, error::Error>;
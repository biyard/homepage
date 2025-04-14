mod error;
mod info;
mod tables;

pub use error::*;
pub use info::*;
pub use tables::*;

pub type Result<T> = std::result::Result<T, error::Error>;

use thiserror::Error;

/// # Summary
///
/// [`Error`](std::error::Error)s referencing [`Store`](crate::Store)s and [`Adapters`].
#[derive(Clone, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
#[error("Failed to parse `{0}` into String. Value: {1}")]
pub struct FromStrError(pub &'static str, pub String);

pub type FromStrResult<T> = std::result::Result<T, FromStrError>;

use std::error::Error;

/// # Summary
///
/// A result which may return any type of [`Error`].
pub type DynamicResult<T> = Result<T, Box<dyn Error>>;

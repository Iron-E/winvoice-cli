use std::error::Error;

/// An [`Error`] type for the crate.
pub type DynResult<T> = Result<T, Box<dyn Error>>;

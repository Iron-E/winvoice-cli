use std::error::Error;

/// An [`Error`] type which can be anything.
pub type DynError = Box<dyn Error>;

/// A [`Result`] which can contain any [`Error`].
pub type DynResult<T> = Result<T, DynError>;

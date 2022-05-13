use thiserror::Error;

/// # Summary
///
/// [`Error`](std::error::Error)s referencing [`Store`](crate::Store)s and [`Adapters`].
#[derive(Clone, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
#[error("An edit made before deserialization is irreconcilable with form prior to serialization")]
pub struct RestoreError;

pub type RestoreResult<T> = std::result::Result<T, RestoreError>;

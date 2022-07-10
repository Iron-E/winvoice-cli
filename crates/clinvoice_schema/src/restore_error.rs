use thiserror::Error;

/// An [`Error`](std::error::Error) for when [`try_restore`](super::RestorableSerde::try_restore)
/// fails, because too much information has been altered between restorations.
#[derive(Clone, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
#[error("An edit is irreconcilable with its original state.")]
pub struct RestoreError;

/// A [`Result`] with [`Err`] equal to [`RestoreError`].
pub type RestoreResult<T> = std::result::Result<T, RestoreError>;

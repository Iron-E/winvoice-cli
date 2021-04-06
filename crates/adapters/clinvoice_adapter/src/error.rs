use
{
	crate::Adapters,

	thiserror::Error,
};

/// # Summary
///
/// [`Error`](std::error::Error)s referencing [`Store`](crate::Store)s and [`Adapters`].
#[derive(Copy, Clone, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
pub enum Error
{
	/// # Summary
	///
	/// An operation requires a [`Store`](crate::Store) with one [kind of adapter][adapter], but a different
	/// [adapter type][adapter] was found.
	///
	/// [adapter]: crate::Adapters
	#[error("Expected the {expected} adapter, but got the {actual} adapter")]
	AdapterMismatch {expected: Adapters, actual: Adapters},

	/// # Summary
	///
	/// The [specified adapter][adapter] type for a [`Store`](crate::Store) was not compiled with
	/// the application.
	///
	/// [adapter]: crate::Adapters
	#[error("Using this adapter requires the {0} feature")]
	FeatureNotFound(Adapters),
}

clinvoice_error::AliasResult!();

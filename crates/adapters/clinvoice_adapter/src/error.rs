use
{
	crate::Adapters,
	snafu::Snafu,
};

/// # Summary
///
/// [`Error`](std::error::Error)s referencing [`Store`](crate::Store)s and [`Adapters`].
#[derive(Debug, Snafu)]
pub enum Error
{
	/// # Summary
	///
	/// An operation requires a [`Store`](crate::Store) with one [kind of adapter][adapter], but a different
	/// [adapter type][adapter] was found.
	///
	/// [adapter]: crate::Adapters
	#[snafu(display("Expected the {} adapter, but got the {} adapter.", expected, actual))]
	AdapterMismatch {expected: Adapters, actual: Adapters},
}

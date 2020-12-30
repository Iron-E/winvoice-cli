use std::error::Error;

use crate::Store;

/// # Summary
///
/// Defines a set of functions which are necessary to adapt across DBMS.
pub trait Adapter<'pass, 'path, 'user, E> where E : Error
{
	/// # Summary
	///
	/// Get the actively focused [`Store`].
	fn active_store(&self) -> &Store<'pass, 'path, 'user>;

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init() -> Result<(), E>;

	/// # Summary
	///
	/// Create a new [`Adapter`].
	///
	/// # Parameters
	///
	/// * `store`, the [`Store`] to use for the [`Adapter`].
	///
	/// # Returns
	///
	/// A new [`Adapter`], that remembers the desired [`Store`].
	fn new(store: Store<'pass, 'path, 'user>) -> Self;
}

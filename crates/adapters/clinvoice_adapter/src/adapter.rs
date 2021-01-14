use crate::{AdapterMismatchError, Store};
use std::error::Error;

/// # Summary
///
/// Defines a set of functions which are necessary to adapt across DBMS.
pub trait Adapter<'pass, 'path, 'user, E> : Sized where E : Error
{
	/// # Summary
	///
	/// Get the actively focused [`Store`].
	fn active_store(&self) -> &Store<'pass, 'path, 'user>;

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(&self) -> Result<(), E>;

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
	fn new<'msg>(store: Store<'pass, 'path, 'user>) -> Result<Self, AdapterMismatchError<'msg>>;
}

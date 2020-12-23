use std::error::Error;

use crate::Connection;

/// # Summary
///
/// Defines a set of functions which are necessary to adapt across DBMS.
pub trait Adapter<'db, 'url, E> where E : Error
{
	/// # Summary
	///
	/// Get the current [`Connection`].
	fn current_connection(self) -> Connection<'db, 'url>;

	/// # Summary
	///
	/// Initialize the database for a given [`Connection`].
	fn init() -> Result<(), E>;

	/// # Summary
	///
	/// Create a new [`Adapter`].
	///
	/// # Parameters
	///
	/// * `connection`, the [`Connection`] to use for the [`Adapter`].
	///
	/// # Returns
	///
	/// A new [`Adapter`], that remembers the desired [`Connection`].
	fn new(connection: Connection<'db, 'url>) -> Self;
}

use std::error::Error;

use crate::Store;

pub trait Initializable
{
	type Error: Error;

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store) -> Result<(), Self::Error>;
}

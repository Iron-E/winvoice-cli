use std::error::Error;

use crate::Store;

#[async_trait::async_trait]
pub trait Initializable
{
	type Error: Error;

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	async fn init(store: &Store) -> Result<(), Self::Error>;
}

use std::error::Error;

use sqlx::{Acquire, Database, Error as SqlxError};

#[async_trait::async_trait]
pub trait Initializable
{
	type Db: Database;
	type Error: Error + From<SqlxError>;

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	async fn init(
		connection: impl 'async_trait + Acquire<'_, Database = Self::Db> + Send,
	) -> Result<(), Self::Error>;
}

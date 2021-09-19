use std::error::Error;
use sqlx::{Database, Executor, Error as SqlxError};

#[async_trait::async_trait]
pub trait Initializable
{
	type Db: Database;
	type Error: Error + From<SqlxError>;

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	async fn init(connection: impl 'async_trait + Executor<'_, Database = Self::Db>) -> Result<(), Self::Error>;
}

use std::error::Error;

#[async_trait::async_trait]
pub trait Initializable
{
	type Db: sqlx::Database;
	type Error: Error + From<sqlx::Error>;

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	async fn init<'conn>(connection: impl sqlx::Executor<'conn, Database = Self::Db>) -> Result<(), Self::Error>;
}

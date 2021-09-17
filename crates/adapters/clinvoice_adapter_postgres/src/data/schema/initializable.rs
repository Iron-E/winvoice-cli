use sqlx::{Postgres, Executor, Error, Result};
use super::PostgresSchema;
use clinvoice_adapter::data::Initializable;

#[async_trait::async_trait]
impl Initializable for PostgresSchema
{
	type Db = Postgres;
	type Error = Error;

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	async fn init(connection: impl Executor<'_, Database = Self::Db>) -> Result<()>
	{
		todo!()
	}
}

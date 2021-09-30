use clinvoice_adapter::data::Initializable;
use sqlx::{Error, Executor, Postgres, Result};

use super::PostgresSchema;

#[async_trait::async_trait]
impl Initializable for PostgresSchema
{
	type Db = Postgres;
	type Error = Error;

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	async fn init(connection: impl 'async_trait + Executor<'_, Database = Self::Db>) -> Result<()>
	{
		sqlx::query!("").await
	}
}

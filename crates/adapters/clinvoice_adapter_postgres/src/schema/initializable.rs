use clinvoice_adapter::Initializable;
use sqlx::{Acquire, Error, Postgres, Result};

use super::PostgresSchema;

#[async_trait::async_trait]
impl Initializable for PostgresSchema
{
	type Db = Postgres;
	type Error = Error;

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	async fn init(
		connection: impl 'async_trait + Acquire<'_, Database = Self::Db> + Send,
	) -> Result<()>
	{
		let mut transaction = connection.begin().await?;

		Self::init_locations(&mut transaction).await?;
		Self::init_people(&mut transaction).await?;
		Self::init_organizations(&mut transaction).await?;
		Self::init_employees(&mut transaction).await?;
		Self::init_contact_info(&mut transaction).await?;
		Self::init_money(&mut transaction).await?;
		Self::init_jobs(&mut transaction).await?;
		Self::init_expenses(&mut transaction).await?;
		Self::init_timesheets(&mut transaction).await?;

		transaction.commit().await
	}
}

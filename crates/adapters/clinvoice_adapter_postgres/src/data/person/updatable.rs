use clinvoice_data::Person;
use sqlx::{Postgres, Executor, Error, Result};

use
{
	super::PostgresPerson,

	clinvoice_adapter::data::Updatable,
};

#[async_trait::async_trait]
impl Updatable for PostgresPerson
{
	type Db = Postgres;
	type Entity = Person;
	type Error = Error;

	async fn update(entity: &Self::Entity, connection: impl Executor<'_, Database = Self::Db>) -> Result<()>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	#[tokio::test]
	async fn update()
	{
		// TODO: write test
	}
}

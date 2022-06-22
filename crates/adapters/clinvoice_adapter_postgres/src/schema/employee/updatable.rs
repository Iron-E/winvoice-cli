use clinvoice_adapter::{schema::columns::EmployeeColumns, Updatable};
use clinvoice_schema::Employee;
use sqlx::{Postgres, Result, Transaction};

use super::PgEmployee;
use crate::PgSchema;

#[async_trait::async_trait]
impl Updatable for PgEmployee
{
	type Db = Postgres;
	type Entity = Employee;

	async fn update<'e, 'i>(
		connection: &mut Transaction<Self::Db>,
		entities: impl 'async_trait + Clone + Iterator<Item = &'i Self::Entity> + Send,
	) -> Result<()>
	where
		'e: 'i,
		Self::Entity: 'e,
	{
		let mut peekable_entities = entities.clone().peekable();

		// There is nothing to do.
		if peekable_entities.peek().is_none()
		{
			return Ok(());
		}

		const COLUMNS: EmployeeColumns<&'static str> = EmployeeColumns::default();
		PgSchema::update(connection, COLUMNS, "employees", "E", |query| {
			query.push_values(peekable_entities, |mut q, e| {
				q.push_bind(e.id)
					.push_bind(&e.name)
					.push_bind(&e.status)
					.push_bind(&e.title);
			});
		})
		.await?;

		Ok(())
	}
}

#[cfg(test)]
mod tests
{
	#[tokio::test]
	async fn update()
	{
		todo!("write test")
	}
}

use clinvoice_adapter::{schema::columns::EmployeeColumns, Updatable};
use clinvoice_schema::Employee;
use sqlx::{Postgres, Result, Transaction};

use super::PgEmployee;
use crate::{schema::PgOrganization, PgSchema};

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
		PgSchema::update(&mut *connection, COLUMNS, "employees", "E", "V", |query| {
			query.push_values(peekable_entities, |mut q, e| {
				q.push_bind(e.id)
					.push_bind(&e.name)
					.push_bind(e.organization.id)
					.push_bind(&e.status)
					.push_bind(&e.title);
			});
		})
		.await?;

		PgOrganization::update(connection, entities.map(|e| &e.organization)).await?;

		Ok(())
	}
}

#[cfg(test)]
mod tests
{
	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn update()
	{
		// TODO: write test
	}
}

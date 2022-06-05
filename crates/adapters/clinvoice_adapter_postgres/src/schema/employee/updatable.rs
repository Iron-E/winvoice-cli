use clinvoice_adapter::{schema::columns::EmployeeColumns, Updatable};
use clinvoice_schema::Employee;
use sqlx::{Postgres, QueryBuilder, Result, Transaction};

use super::PgEmployee;
use crate::schema::PgOrganization;

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
		const COLUMNS: EmployeeColumns<&'static str> = EmployeeColumns::default();
		const TABLE_IDENT: &str = "E";
		const VALUES_IDENT: &str = "V";

		let mut peekable_entities = entities.clone().peekable();

		// There is nothing to do.
		if peekable_entities.peek().is_none()
		{
			return Ok(());
		}

		let values_columns = COLUMNS.scoped(VALUES_IDENT);

		let mut query = QueryBuilder::new("UPDATE employees AS ");

		query
			.separated(' ')
			.push(TABLE_IDENT)
			.push("SET")
			.push(COLUMNS.name)
			.push_unseparated('=')
			.push_unseparated(values_columns.name)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.status)
			.push_unseparated('=')
			.push_unseparated(values_columns.status)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.organization_id)
			.push_unseparated('=')
			.push_unseparated(values_columns.organization_id)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.title)
			.push_unseparated('=')
			.push_unseparated(values_columns.title)
			.push("FROM (");

		query.push_values(peekable_entities, |mut q, e| {
			q.push_bind(e.id)
				.push_bind(&e.name)
				.push_bind(e.organization.id)
				.push_bind(&e.status)
				.push_bind(&e.title);
		});

		query
			.separated(' ')
			.push(") AS")
			.push(VALUES_IDENT)
			.push('(')
			.push_unseparated(COLUMNS.id)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.name)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.organization_id)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.status)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.title)
			.push_unseparated(')')
			.push("WHERE")
			.push(COLUMNS.scoped(TABLE_IDENT).id)
			.push_unseparated('=')
			.push_unseparated(values_columns.id);

		query.push(';').build().execute(&mut *connection).await?;

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

use clinvoice_adapter::{Updatable, schema::columns::ExpenseColumns};
use clinvoice_schema::Expense;
use sqlx::{Postgres, Result, Transaction, QueryBuilder};

use super::PgExpenses;

#[async_trait::async_trait]
impl Updatable for PgExpenses
{
	type Db = Postgres;
	type Entity = Expense;

	async fn update<'e, 'i>(
		connection: &mut Transaction<Self::Db>,
		entities: impl 'async_trait + Clone + Iterator<Item = &'i Self::Entity> + Send,
	) -> Result<()>
	where
		'e: 'i,
		Self::Entity: 'e,
	{
		const COLUMNS: ExpenseColumns<&'static str> = ExpenseColumns::default();
		const TABLE_IDENT: &'static str = "X";
		const VALUES_IDENT: &'static str = "V";

		let mut peekable_entities = entities.clone().peekable();

		// There is nothing to do.
		if peekable_entities.peek().is_none()
		{
			return Ok(());
		}

		let values_columns = COLUMNS.scoped(VALUES_IDENT);

		let mut query = QueryBuilder::new("UPDATE expenses AS ");

		query
			.separated(' ')
			.push(TABLE_IDENT)
			.push("SET")
			.push(COLUMNS.category)
			.push_unseparated('=')
			.push_unseparated(values_columns.category)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.cost)
			.push_unseparated('=')
			.push_unseparated(values_columns.cost)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.description)
			.push_unseparated('=')
			.push_unseparated(values_columns.description)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.timesheet_id)
			.push_unseparated('=')
			.push_unseparated(values_columns.timesheet_id)
			.push("FROM (");

		query.push_values(peekable_entities, |mut q, e| {
			q.push_bind(&e.category)
				.push_bind(e.cost.amount.to_string())
				.push_bind(&e.description)
				.push_bind(e.id)
				.push_bind(e.timesheet_id);
		});

		query
			.separated(' ')
			.push(") AS")
			.push(VALUES_IDENT)
			.push('(')
			.push_unseparated(COLUMNS.category)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.cost)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.description)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.id)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.timesheet_id)
			.push_unseparated(')')
			.push("WHERE")
			.push(COLUMNS.scoped(TABLE_IDENT).id)
			.push_unseparated('=')
			.push_unseparated(values_columns.id);

		query.push(';').build().execute(connection).await?;

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

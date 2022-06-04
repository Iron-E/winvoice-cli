use clinvoice_adapter::{Updatable, schema::columns::TimesheetColumns};
use clinvoice_schema::Timesheet;
use sqlx::{Postgres, Result, Transaction, QueryBuilder};

use crate::schema::{PgEmployee, PgJob};

use super::PgTimesheet;

#[async_trait::async_trait]
impl Updatable for PgTimesheet
{
	type Db = Postgres;
	type Entity = Timesheet;

	async fn update<'e, 'i>(
		connection: &mut Transaction<Self::Db>,
		entities: impl 'async_trait + Clone + Iterator<Item = &'i Self::Entity> + Send,
	) -> Result<()>
	where
		'e: 'i,
		Self::Entity: 'e,
	{
		const COLUMNS: TimesheetColumns<&'static str> = TimesheetColumns::default();
		const TABLE_IDENT: &'static str = "O";
		const VALUES_IDENT: &'static str = "V";

		let mut peekable_entities = entities.clone().peekable();

		// There is nothing to do.
		if peekable_entities.peek().is_none()
		{
			return Ok(());
		}

		let values_columns = COLUMNS.scoped(VALUES_IDENT);

		let mut query = QueryBuilder::new("UPDATE timesheets AS ");

		query
			.separated(' ')
			.push(TABLE_IDENT)
			.push("SET")
			.push(COLUMNS.employee_id)
			.push_unseparated('=')
			.push_unseparated(values_columns.employee_id)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.job_id)
			.push_unseparated('=')
			.push_unseparated(values_columns.job_id)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.time_begin)
			.push_unseparated('=')
			.push_unseparated(values_columns.time_begin)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.time_end)
			.push_unseparated('=')
			.push_unseparated(values_columns.time_end)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.work_notes)
			.push_unseparated('=')
			.push_unseparated(values_columns.work_notes)
			.push("FROM (");

		query.push_values(peekable_entities, |mut q, e| {
			q.push_bind(e.employee.id)
				.push_bind(e.id)
				.push_bind(e.job.id)
				.push_bind(e.time_begin)
				.push_bind(e.time_end)
				.push_bind(&e.work_notes);
		});

		query
			.separated(' ')
			.push(") AS")
			.push(VALUES_IDENT)
			.push('(')
			.push_unseparated(COLUMNS.employee_id)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.id)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.job_id)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.time_begin)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.time_end)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.work_notes)
			.push_unseparated(')')
			.push("WHERE")
			.push(COLUMNS.scoped(TABLE_IDENT).id)
			.push_unseparated('=')
			.push_unseparated(values_columns.id);

		query.push(';').build().execute(&mut *connection).await?;

		let employees = entities.clone().map(|e| &e.employee);
		PgEmployee::update(connection, employees).await?;
		PgJob::update(connection, entities.map(|e| &e.job)).await?;

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

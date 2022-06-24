use clinvoice_adapter::{schema::columns::TimesheetColumns, Updatable};
use clinvoice_schema::{Expense, Timesheet};
use sqlx::{Postgres, Result, Transaction};

use super::PgTimesheet;
use crate::{
	schema::{PgEmployee, PgExpenses, PgJob},
	PgSchema,
};

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
		let mut peekable_entities = entities.clone().peekable();

		// There is nothing to do.
		if peekable_entities.peek().is_none()
		{
			return Ok(());
		}

		const COLUMNS: TimesheetColumns<&'static str> = TimesheetColumns::default();
		PgSchema::update(connection, COLUMNS, "locations", "L", |query| {
			query.push_values(peekable_entities, |mut q, e| {
				q.push_bind(e.employee.id)
					.push_bind(e.id)
					.push_bind(e.job.id)
					.push_bind(e.time_begin)
					.push_bind(e.time_end)
					.push_bind(&e.work_notes);
			});
		})
		.await?;

		let employees = entities.clone().map(|e| &e.employee);

		// TODO: use `for<'a> |e: &'a Timesheet| &t.expenses`
		let expenses = entities.clone().map(mapper).flatten();
		fn mapper(t: &Timesheet) -> &[Expense]
		{
			&t.expenses
		}

		PgEmployee::update(connection, employees).await?;
		PgExpenses::update(connection, expenses).await?;
		PgJob::update(connection, entities.map(|e| &e.job)).await
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

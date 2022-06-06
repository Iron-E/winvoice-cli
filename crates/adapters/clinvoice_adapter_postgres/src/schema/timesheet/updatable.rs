use clinvoice_adapter::{
	schema::columns::TimesheetColumns,
	Updatable,
};
use clinvoice_schema::Timesheet;
use sqlx::{Postgres, Result, Transaction};

use super::PgTimesheet;
use crate::{schema::{PgEmployee, PgJob}, PgSchema};

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
		PgSchema::update(&mut *connection, COLUMNS, "locations", "L", "V", |query| {
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

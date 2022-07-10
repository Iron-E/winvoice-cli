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

		PgSchema::update(connection, EmployeeColumns::default(), |query| {
			query.push_values(peekable_entities, |mut q, e| {
				q.push_bind(e.id)
					.push_bind(&e.name)
					.push_bind(&e.status)
					.push_bind(&e.title);
			});
		})
		.await
	}
}

#[cfg(test)]
mod tests
{
	use clinvoice_adapter::{schema::EmployeeAdapter, Updatable};
	use clinvoice_match::MatchEmployee;
	use pretty_assertions::assert_eq;

	use crate::schema::{util, PgEmployee};

	#[tokio::test]
	async fn update()
	{
		let connection = util::connect().await;

		let mut employee = PgEmployee::create(
			&connection,
			"My Name".into(),
			"Employed".into(),
			"Janitor".into(),
		)
		.await
		.unwrap();

		employee.name = format!("Not {}", employee.name);
		employee.status = format!("Not {}", employee.status);
		employee.title = format!("Not {}", employee.title);

		{
			let mut transaction = connection.begin().await.unwrap();
			PgEmployee::update(&mut transaction, [&employee].into_iter())
				.await
				.unwrap();
			transaction.commit().await.unwrap();
		}

		let db_employee = PgEmployee::retrieve(&connection, &MatchEmployee {
			id: employee.id.into(),
			..Default::default()
		})
		.await
		.unwrap()
		.pop()
		.unwrap();

		assert_eq!(employee, db_employee);
	}
}

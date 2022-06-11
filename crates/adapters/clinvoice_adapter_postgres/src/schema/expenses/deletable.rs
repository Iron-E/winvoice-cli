use clinvoice_adapter::Deletable;
use clinvoice_schema::{Expense, Id};
use sqlx::{Executor, Postgres, Result};

use super::PgExpenses;
use crate::PgSchema;

#[async_trait::async_trait]
impl Deletable for PgExpenses
{
	type Db = Postgres;
	type Entity = Expense;

	async fn delete<'e, 'i>(
		connection: impl 'async_trait + Executor<'_, Database = Self::Db>,
		entities: impl 'async_trait + Iterator<Item = &'i Self::Entity> + Send,
	) -> Result<()>
	where
		'e: 'i,
		Self::Entity: 'e,
	{
		// TODO: use `for<'a> |x: &'a Expense| x.id`
		fn mapper(x: &Expense) -> Id
		{
			x.id
		}

		PgSchema::delete(connection, "expenses", entities.map(mapper)).await
	}
}

#[cfg(test)]
mod tests
{
	use core::time::Duration;

	use clinvoice_adapter::{
		schema::{
			EmployeeAdapter,
			ExpensesAdapter,
			JobAdapter,
			LocationAdapter,
			OrganizationAdapter,
			TimesheetAdapter,
		},
		Deletable,
	};
	use clinvoice_finance::{Currency, ExchangeRates, Exchangeable, Money};
	use clinvoice_match::{MatchExpense, MatchSet};
	use clinvoice_schema::{
		chrono::{TimeZone, Utc},
		Invoice,
	};

	use crate::schema::{
		util,
		PgEmployee,
		PgExpenses,
		PgJob,
		PgLocation,
		PgOrganization,
		PgTimesheet,
	};

	// TODO: use fuzzing
	#[tokio::test]
	async fn delete()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into(), None)
			.await
			.unwrap();

		let organization =
			PgOrganization::create(&connection, Vec::new(), earth, "Some Organization".into())
				.await
				.unwrap();

		let employee = PgEmployee::create(
			&connection,
			"My Name".into(),
			organization.clone(),
			"Employed".into(),
			"Janitor".into(),
		)
		.await
		.unwrap();

		let job = PgJob::create(
			&connection,
			organization.clone(),
			None,
			Utc.ymd(1990, 07, 12).and_hms(14, 10, 00),
			Duration::from_secs(900),
			Invoice {
				date: None,
				hourly_rate: Money::new(20_00, 2, Currency::USD),
			},
			String::new(),
			"Do something".into(),
		)
		.await
		.unwrap();

		let timesheet = PgTimesheet::create(
			&connection,
			employee.clone(),
			vec![
				(
					"Flight".into(),
					Money::new(300_56, 2, Currency::JPY),
					"Trip to Hawaii for research".into(),
				),
				(
					"Food".into(),
					Money::new(10_17, 2, Currency::USD),
					"Takeout".into(),
				),
				(
					"Taxi".into(),
					Money::new(563_30, 2, Currency::NOK),
					"Took a taxi cab".into(),
				),
			],
			job,
			Utc.ymd(2022, 06, 08).and_hms(15, 27, 00),
			Some(Utc.ymd(2022, 06, 09).and_hms(07, 00, 00)),
		)
		.await
		.unwrap();

		PgExpenses::delete(
			&connection,
			[&timesheet.expenses[0], &timesheet.expenses[1]].into_iter(),
		)
		.await
		.unwrap();

		let exchange_rates = ExchangeRates::new().await.unwrap();

		assert_eq!(
			PgExpenses::retrieve(
				&connection,
				&MatchSet::Contains(MatchExpense {
					timesheet_id: timesheet.id.into(),
					..Default::default()
				})
			)
			.await
			.unwrap()[&timesheet.id]
				.as_slice(),
			&[timesheet.expenses[2].exchange_ref(Default::default(), &exchange_rates)],
		);
	}
}

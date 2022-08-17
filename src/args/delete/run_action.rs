use core::fmt::Display;
use std::error::Error;

use clinvoice_adapter::{
	schema::{
		ContactAdapter,
		EmployeeAdapter,
		ExpensesAdapter,
		JobAdapter,
		LocationAdapter,
		OrganizationAdapter,
		TimesheetAdapter,
	},
	Deletable,
	Retrievable,
};
use clinvoice_config::Config;
use serde::{de::DeserializeOwned, Serialize};
use sqlx::{Database, Executor, Pool};

use super::{Delete, DeleteCommand};
use crate::{args::RunAction, fmt, input, utils::Identifiable, DynResult};

#[async_trait::async_trait(?Send)]
impl RunAction for Delete
{
	async fn action<CAdapter, EAdapter, JAdapter, LAdapter, OAdapter, TAdapter, XAdapter, Db>(
		self,
		connection: Pool<Db>,
		_config: Config,
	) -> DynResult<()>
	where
		CAdapter: Deletable<Db = Db> + ContactAdapter,
		EAdapter: Deletable<Db = Db> + EmployeeAdapter,
		JAdapter: Deletable<Db = Db> + JobAdapter,
		LAdapter: Deletable<Db = Db> + LocationAdapter,
		OAdapter: Deletable<Db = Db> + OrganizationAdapter,
		TAdapter: Deletable<Db = Db> + TimesheetAdapter,
		XAdapter: Deletable<Db = Db> + ExpensesAdapter,
		Db: Database,
		for<'connection> &'connection mut Db::Connection: Executor<'connection, Database = Db>,
	{
		/// A generic deletion function which works for any of the provided adapters in the outer
		/// function, as they all implement `DelRetrievable` at the minimum.
		async fn del<DelRetrievable, Db, Match>(
			connection: &Pool<Db>,
			match_condition: Match,
		) -> DynResult<()>
		where
			Db: Database,
			Match: TryInto<Option<DelRetrievable::Match>>,
			Match::Error: 'static + Error,
			DelRetrievable: Deletable<Db = Db>,
			<DelRetrievable as Deletable>::Entity: Clone + Display + Identifiable + Sync,
			DelRetrievable: Retrievable<Db = Db, Entity = <DelRetrievable as Deletable>::Entity>,
			DelRetrievable::Match: Default + DeserializeOwned + Serialize,
			for<'connection> &'connection mut Db::Connection: Executor<'connection, Database = Db>,
		{
			let match_condition = match_condition.try_into()?;
			let type_name = fmt::type_name::<<DelRetrievable as Deletable>::Entity>();
			let retrieved = input::select_retrieved::<DelRetrievable, _, _>(
				connection,
				match_condition,
				format!("Query the {type_name} to delete"),
			)
			.await?;

			let selected = input::select(&retrieved, format!("Select the {type_name} to delete"))?;
			DelRetrievable::delete(
				connection,
				selected.iter().inspect(|s| Delete::report_deleted(*s)),
			)
			.await?;
			Ok(())
		}

		match self.command
		{
			DeleteCommand::Contact => del::<CAdapter, _, _>(&connection, self.match_args).await,
			DeleteCommand::Employee => del::<EAdapter, _, _>(&connection, self.match_args).await,
			DeleteCommand::Expense => del::<XAdapter, _, _>(&connection, self.match_args).await,
			DeleteCommand::Job => del::<JAdapter, _, _>(&connection, self.match_args).await,
			DeleteCommand::Location => del::<LAdapter, _, _>(&connection, self.match_args).await,
			DeleteCommand::Organization =>
			{
				del::<OAdapter, _, _>(&connection, self.match_args).await
			},
			DeleteCommand::Timesheet => del::<TAdapter, _, _>(&connection, self.match_args).await,
		}
	}
}

#[cfg(all(feature = "postgres", test))]
mod tests
{
	use core::time::Duration;

	use clinvoice_adapter::{
		schema::{
			ContactAdapter,
			EmployeeAdapter,
			ExpensesAdapter,
			JobAdapter,
			LocationAdapter,
			OrganizationAdapter,
			TimesheetAdapter,
		},
		Retrievable,
	};
	use clinvoice_adapter_postgres::schema::{
		PgContact,
		PgEmployee,
		PgExpenses,
		PgJob,
		PgLocation,
		PgOrganization,
		PgTimesheet,
	};
	use clinvoice_config::Config;
	use clinvoice_match::{
		MatchContact,
		MatchEmployee,
		MatchExpense,
		MatchJob,
		MatchLocation,
		MatchOrganization,
		MatchTimesheet,
	};
	use clinvoice_schema::{chrono::Utc, ContactKind::Other, Currency, Invoice, Money};
	use pretty_assertions::assert_eq;
	use serde_yaml as yaml;
	use sqlx::PgPool;

	use super::{Delete, DeleteCommand, RunAction};
	use crate::utils;

	/// WARN: use `cargo test -- --test-threads=1`.
	#[tokio::test]
	async fn run_action()
	{
		let database_url = utils::database_url().unwrap();
		let connection_fut = PgPool::connect(&database_url);

		let config: Config = toml::from_str(&format!(
			"[jobs]
			default_increment = '15min'

			[invoices]
			default_currency = 'USD'

			[employees]

			[organizations]

			[stores.default]
			adapter = 'postgres'
			url = '{database_url}'",
		))
		.unwrap();

		let filepath = utils::temp_file::<Delete>("run-action");
		let run = |config: Config, command: DeleteCommand| {
			let filepath = filepath.clone();
			async move {
				Delete { command, match_args: Some(filepath).into(), store_args: "default".into() }
					.run(config)
					.await
					.unwrap()
			}
		};

		/* ########## `clinvoice delete employee` ########## */

		let connection = connection_fut.await.unwrap();

		// {{{
		let employee =
			PgEmployee::create(&connection, "bob".into(), "bob status".into(), "bob title".into())
				.await
				.unwrap();

		let match_employee = MatchEmployee::from(employee.id);
		utils::write_yaml(&filepath, &match_employee);

		run(config.clone(), DeleteCommand::Employee).await;
		assert_eq!(PgEmployee::retrieve(&connection, match_employee).await.unwrap(), Vec::new());
		// }}}

		/* ########## `clinvoice delete location` ########## */

		// {{{
		let location = PgLocation::create(&connection, "location".into(), None).await.unwrap();

		let match_location = MatchLocation::from(location.id);
		utils::write_yaml(&filepath, &match_location);

		run(config.clone(), DeleteCommand::Location).await;
		assert_eq!(PgLocation::retrieve(&connection, match_location).await.unwrap(), Vec::new());
		// }}}

		/* ########## `clinvoice delete contact` ########## */

		// {{{
		let contact =
			PgContact::create(&connection, Other("Email".into()), "Preferred Contact".into())
				.await
				.unwrap();

		let match_contact = MatchContact::from(contact.label);
		utils::write_yaml(&filepath, &match_contact);

		run(config.clone(), DeleteCommand::Contact).await;
		assert_eq!(PgContact::retrieve(&connection, match_contact).await.unwrap(), Vec::new());
		// }}}

		/* ########## `clinvoice delete organization` ########## */

		// {{{
		let organization =
			PgOrganization::create(&connection, location, "Foo".into()).await.unwrap();

		let match_organization = MatchOrganization::from(organization.id);
		utils::write_yaml(&filepath, &match_organization);

		run(config.clone(), DeleteCommand::Organization).await;
		assert_eq!(
			PgOrganization::retrieve(&connection, match_organization).await.unwrap(),
			Vec::new()
		);
		// }}}

		/* ########## `clinvoice delete job` ########## */

		// {{{
		let job = PgJob::create(
			&connection,
			organization,
			None,
			Utc::now(),
			Duration::from_secs(300),
			Invoice { hourly_rate: Money::new(17_60, 2, Currency::Usd), date: None },
			"Placeholder".into(),
			"Objectives".into(),
		)
		.await
		.unwrap();

		let match_job = MatchJob::from(job.id);
		utils::write_yaml(&filepath, &match_job);

		run(config.clone(), DeleteCommand::Job).await;
		assert_eq!(PgJob::retrieve(&connection, match_job).await.unwrap(), Vec::new());
		// }}}

		/* ########## `clinvoice delete timesheet` ########## */

		// {{{
		let mut transaction = connection.begin().await.unwrap();
		let timesheet = PgTimesheet::create(
			&mut transaction,
			employee,
			Vec::new(),
			job,
			Utc::now(),
			None,
			"Notes".into(),
		)
		.await
		.unwrap();
		transaction.commit().await.unwrap();

		let match_timesheet = MatchTimesheet::from(timesheet.id);
		utils::write_yaml(&filepath, &match_timesheet);

		run(config.clone(), DeleteCommand::Timesheet).await;
		assert_eq!(PgTimesheet::retrieve(&connection, match_timesheet).await.unwrap(), Vec::new());
		// }}}

		/* ########## `clinvoice delete expense` ########## */

		// {{{
		let expense = PgExpenses::create(
			&connection,
			vec![("Category".into(), Money::new(2, 0, Default::default()), "Desc".into())],
			timesheet.id,
		)
		.await
		.map(|mut v| v.remove(0))
		.unwrap();

		let match_expense = MatchExpense::from(expense.id);
		utils::write_yaml(&filepath, &match_expense);

		run(config.clone(), DeleteCommand::Expense).await;
		assert_eq!(PgExpenses::retrieve(&connection, match_expense).await.unwrap(), Vec::new());
		// }}}
	}
}

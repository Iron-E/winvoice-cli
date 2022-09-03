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

			let selected = match cfg!(test)
			{
				false => input::select(retrieved, format!("Select the {type_name} to delete"))?,
				true => retrieved,
			};

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
	use core::{fmt::Debug, time::Duration};
	use std::path::PathBuf;

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
	use clinvoice_schema::{chrono::Utc, ContactKind::Other, Currency, Invoice, Money};
	use pretty_assertions::assert_eq;
	use serde::Serialize;
	use sqlx::{PgPool, Postgres};

	use super::{Delete, DeleteCommand, RunAction};
	use crate::utils;

	/// WARN: must use `cargo test -- --test-threads=1`.
	#[tokio::test]
	async fn run_action()
	{
		/// Runs the given `command`, then [assert](pretty_assertions)s that there are no rows in
		/// the database matching the `condition` derived `from` the value specified.
		async fn assert<R, T>(
			connection: &PgPool,
			command: DeleteCommand,
			config: Config,
			from: T,
			filepath: PathBuf,
		) where
			R: Retrievable<Db = Postgres>,
			R::Entity: Debug + PartialEq,
			R::Match: From<T> + Serialize,
		{
			let condition = R::Match::from(from);
			utils::write_yaml(&filepath, &condition);
			Delete { command, match_args: Some(filepath).into(), store_args: "default".into() }
				.run(config)
				.await
				.unwrap();

			assert_eq!(R::retrieve(&connection, condition).await.unwrap(), []);
		}

		let database_url = utils::database_url().unwrap();
		let connection_fut = PgPool::connect(&database_url);

		let filepath = utils::temp_file::<Delete>("run-action");
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

		/* Setup {{{ */
		let connection = connection_fut.await.unwrap();

		let (contact, employee, location) = futures::try_join!(
			PgContact::create(&connection, Other("Email".into()), "Preferred Contact".into()),
			PgEmployee::create(&connection, "bob".into(), "bob status".into(), "bob title".into()),
			PgLocation::create(&connection, "location".into(), None),
		)
		.unwrap();

		let (contact_label, employee_id, location_id) =
			(contact.label.clone(), employee.id, location.id);

		let organization =
			PgOrganization::create(&connection, location, "Foo".into()).await.unwrap();
		let organization_id = organization.id;

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
		let job_id = job.id;

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

		let expense = PgExpenses::create(
			&connection,
			vec![("Category".into(), Money::new(2, 0, Default::default()), "Desc".into())],
			timesheet.id,
		)
		.await
		.map(|mut v| v.remove(0))
		.unwrap();

		/* }}}
		 * Tests {{{ */
		assert::<PgExpenses, _>(
			&connection,
			DeleteCommand::Expense,
			config.clone(),
			expense.id,
			filepath.clone(),
		)
		.await;

		assert::<PgTimesheet, _>(
			&connection,
			DeleteCommand::Timesheet,
			config.clone(),
			timesheet.id,
			filepath.clone(),
		)
		.await;

		assert::<PgJob, _>(
			&connection,
			DeleteCommand::Job,
			config.clone(),
			job_id,
			filepath.clone(),
		)
		.await;

		assert::<PgOrganization, _>(
			&connection,
			DeleteCommand::Organization,
			config.clone(),
			organization_id,
			filepath.clone(),
		)
		.await;

		futures::join!(
			assert::<PgContact, _>(
				&connection,
				DeleteCommand::Contact,
				config.clone(),
				contact_label,
				filepath.clone(),
			),
			assert::<PgEmployee, _>(
				&connection,
				DeleteCommand::Employee,
				config.clone(),
				employee_id,
				filepath.clone(),
			),
			assert::<PgLocation, _>(
				&connection,
				DeleteCommand::Location,
				config,
				location_id,
				filepath,
			),
		);
		/* }}} */
	}
}

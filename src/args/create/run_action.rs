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
};
use clinvoice_config::Config;
use clinvoice_schema::{chrono::Utc, ContactKind, Invoice, InvoiceDate};
use sqlx::{Database, Executor, Pool, Transaction};

use super::{Create, CreateCommand};
use crate::{
	args::{match_args::MatchArgs, update::Update, RunAction},
	input,
	utils,
	DynResult,
};

#[async_trait::async_trait(?Send)]
impl RunAction for Create
{
	async fn action<CAdapter, EAdapter, JAdapter, LAdapter, OAdapter, TAdapter, XAdapter, Db>(
		self,
		connection: Pool<Db>,
		config: Config,
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
		for<'connection> &'connection mut Transaction<'connection, Db>:
			Executor<'connection, Database = Db>,
	{
		match self.command
		{
			CreateCommand::Contact { label, address, email, phone, info } =>
			{
				let kind = match (address.flag(), email, phone)
				{
					(true, ..) =>
					{
						let match_condition = MatchArgs::from(address.argument()).try_into()?;
						input::select_one_retrieved::<LAdapter, _, _>(
							&connection,
							match_condition,
							"Query the Location of this address",
						)
						.await
						.map(ContactKind::Address)?
					},

					(_, true, _) => ContactKind::Email(info),
					(.., true) => ContactKind::Phone(info),
					(false, false, false) => ContactKind::Other(info),
				};

				let created = CAdapter::create(&connection, kind, label).await?;
				Self::report_created(&created);
			},

			CreateCommand::Employee { name, status, title } =>
			{
				let created = EAdapter::create(&connection, name, status, title).await?;
				Self::report_created(&created);
			},

			CreateCommand::Expense { category, cost, description, timesheet } =>
			{
				let match_timesheet = MatchArgs::from(timesheet).try_into()?;
				let selected = input::select_one_retrieved::<TAdapter, _, _>(
					&connection,
					match_timesheet,
					"Query the Timesheet this Expense is for",
				)
				.await?;

				#[rustfmt::skip]
				let created = XAdapter::create(&connection, vec![(category, cost, description)], selected.id)
					.await
					.map(|mut v| v.pop().expect("at least one `Expense` should have been created"))?;

				Self::report_created(&created);
			},

			CreateCommand::Job {
				client,
				date_close,
				date_invoice_issued,
				date_invoice_paid,
				date_open,
				employer,
				hourly_rate,
				increment,
				notes,
				objectives,
			} =>
			{
				let match_client = match employer
				{
					false => MatchArgs::from(client).try_into()?,
					true => config.organizations.employer_id_or_err().map(|id| Some(id.into()))?,
				};

				let selected = input::select_one_retrieved::<OAdapter, _, _>(
					&connection,
					match_client,
					"Query the client Organization for this Job",
				)
				.await?;

				let created = JAdapter::create(
					&connection,
					selected,
					date_close.map(utils::naive_local_datetime_to_utc),
					date_open.map_or_else(Utc::now, utils::naive_local_datetime_to_utc),
					increment.unwrap_or(config.jobs.default_increment),
					Invoice {
						date: date_invoice_issued.map(|issued| InvoiceDate {
							issued: utils::naive_local_datetime_to_utc(issued),
							paid: date_invoice_paid.map(utils::naive_local_datetime_to_utc),
						}),
						hourly_rate,
					},
					notes,
					objectives,
				)
				.await?;

				Self::report_created(&created);
			},

			CreateCommand::Location { inside, outside, names } =>
			{
				let mut names_reversed = names.into_iter().rev();

				let final_name = names_reversed.next().expect(
					"clap config should have ensured that `names` has length of at least one",
				);

				let outside_of_final = match inside.flag()
				{
					false => None,
					true =>
					{
						let match_condition = MatchArgs::from(inside.argument()).try_into()?;
						input::select_one_retrieved::<LAdapter, _, _>(
							&connection,
							match_condition,
							format!("Query the Location outside of {final_name}"),
						)
						.await
						.map(Some)?
					},
				};

				// {{{
				let mut transaction = connection.begin().await?;

				// TODO: convert to `try_fold` after `stream`s merge to `std`? {{{2
				// TODO: use `inspect` after rust-lang/rust#91345
				let mut l =
					LAdapter::create(&mut *transaction, final_name, outside_of_final).await?;
				Self::report_created(&l);
				for n in names_reversed
				{
					l = LAdapter::create(&mut *transaction, n, Some(l)).await?;
					Self::report_created(&l);
				}
				// 2}}}

				let created = l;

				if outside.flag()
				{
					let match_condition = MatchArgs::from(outside.argument()).try_into()?;
					let mut inside_locations = input::select_retrieved::<LAdapter, _, _>(
						&connection,
						match_condition,
						format!("Query Locations that are inside {created}"),
					)
					.await?;

					// PERF: only call `.clone` on the newly-`created` `Location` for elements in
					//       `inside_locations` other than the first
					if let Some(after_first) = inside_locations.get_mut(1..)
					{
						after_first.iter_mut().for_each(|mut l| {
							l.outer = Some(created.clone().into());
						})
					}

					if let Some(first) = inside_locations.first_mut()
					{
						first.outer = Some(created.into());
					}

					#[rustfmt::skip]
					LAdapter::update(
						&mut transaction,
						inside_locations.iter().inspect(|l| Update::report_updated(*l)),
					)
					.await?;
				}

				transaction.commit().await?;
				// }}}
			},

			CreateCommand::Organization { location, name } =>
			{
				let match_location = MatchArgs::from(location).try_into()?;
				let selected = input::select_one_retrieved::<LAdapter, _, _>(
					&connection,
					match_location,
					"Query the Location of this Organization",
				)
				.await?;

				let created = OAdapter::create(&connection, selected, name).await?;
				Self::report_created(&created);
			},

			CreateCommand::Timesheet {
				default_employee,
				employee,
				job,
				time_begin,
				time_end,
				work_notes,
			} =>
			{
				let match_employee = match default_employee
				{
					false => MatchArgs::from(employee).try_into()?,
					true => config.employees.id_or_err().map(|id| Some(id.into()))?,
				};

				let employee = input::select_one_retrieved::<EAdapter, _, _>(
					&connection,
					match_employee,
					"Query the Employee who is responsible for the work",
				)
				.await?;

				let match_job = MatchArgs::from(job).try_into()?;
				let job = input::select_one_retrieved::<JAdapter, _, _>(
					&connection,
					match_job,
					"Query the Job being worked on",
				)
				.await?;

				let expenses = match cfg!(test) || time_end.is_none()
				{
					false => input::expense::menu()?,
					true => Vec::new(),
				};

				// {{{
				let mut transaction = connection.begin().await?;

				let created = TAdapter::create(
					&mut transaction,
					employee,
					expenses,
					job,
					time_begin.map_or_else(Utc::now, utils::naive_local_datetime_to_utc),
					time_end.map(utils::naive_local_datetime_to_utc),
					work_notes.unwrap_or_default(),
				)
				.await?;

				transaction.commit().await?;
				// }}}

				Self::report_created(&created);
			},
		};

		Ok(())
	}
}

#[cfg(all(feature = "postgres", test))]
mod tests
{
	use clinvoice_adapter::{
		fmt::{sql, QueryBuilderExt, TableToSql},
		schema::{
			columns::{
				EmployeeColumns,
				ExpenseColumns,
				JobColumns,
				LocationColumns,
				OrganizationColumns,
				TimesheetColumns,
			},
			LocationAdapter,
		},
		Deletable,
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
		MatchEmployee,
		MatchJob,
		MatchLocation,
		MatchOrganization,
		MatchTimesheet,
	};
	use clinvoice_schema::{
		chrono::{DateTime, Duration, Local, NaiveDate, Utc},
		Currency,
		Id,
		Invoice,
		InvoiceDate,
		Money,
	};
	use money2::{Exchange, ExchangeRates};
	use pretty_assertions::assert_eq;
	use sqlx::{
		postgres::{PgPool, Postgres},
		QueryBuilder,
		Result,
		Row,
	};

	use super::{Create, CreateCommand, RunAction};
	use crate::{args::flag_or_argument::FlagOrArgument, utils};

	/// WARN: use `cargo test -- --test-threads=1`.
	#[tokio::test]
	async fn run_action()
	{
		async fn contact(
			connection: &PgPool,
			label: &str,
		) -> Result<<PgContact as Retrievable>::Entity>
		{
			PgContact::retrieve(
				connection,
				<PgContact as Retrievable>::Match::from(label.to_owned()),
			)
			.await
			.map(|mut v| v.remove(0))
		}

		/// Retrieve the most-recently-created row in the database as its structural counterpart.
		async fn latest_entity<R, T>(connection: &PgPool) -> Result<R::Entity>
		where
			R: Retrievable<Db = Postgres>,
			R::Match: From<Id>,
			T: TableToSql,
		{
			let id = QueryBuilder::new(sql::SELECT)
				.push('*')
				.push_default_from::<T>()
				.push(sql::ORDER_BY)
				.push("id")
				.push(sql::DESCENDING)
				.push(sql::LIMIT)
				.push(1)
				.prepare()
				.fetch_one(connection)
				.await
				.and_then(|row| row.try_get::<Id, _>("id"))?;

			R::retrieve(connection, R::Match::from(id)).await.map(|mut v| v.remove(0))
		}

		/// Run a [`Create`] `command`.
		async fn run(config: Config, command: CreateCommand)
		{
			Create { command, store_args: "default".into() }.run(config).await.unwrap()
		}

		let database_url = utils::database_url().unwrap();
		let connection_fut = PgPool::connect(&database_url);
		let exchange_rates_fut = ExchangeRates::new();
		let mut config: Config = toml::from_str(&format!(
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

		/* ########## `clinvoice create employee` ########## */

		// {{{
		let name = "bob";
		let status = "bob status";
		let title = "bob title";

		run(config.clone(), CreateCommand::Employee {
			name: name.into(),
			status: status.into(),
			title: title.into(),
		})
		.await;

		let connection = connection_fut.await.unwrap();

		let most_recent =
			latest_entity::<PgEmployee, EmployeeColumns<&str>>(&connection).await.unwrap();

		assert_eq!(most_recent.name, name);
		assert_eq!(most_recent.status, status);
		assert_eq!(most_recent.title, title);
		// }}}

		config.employees.id = Some(most_recent.id);

		/* ########## `clinvoice create location` ########## */

		// {{{
		let (arizona, usa) = ("Arizona", "USA");
		let names = vec![arizona.to_owned(), usa.to_owned()];

		run(config.clone(), CreateCommand::Location {
			inside: FlagOrArgument::Flag(false),
			names: names.clone(),
			outside: FlagOrArgument::Flag(false),
		})
		.await;

		let most_recent =
			latest_entity::<PgLocation, LocationColumns<&str>>(&connection).await.unwrap();

		assert_eq!(most_recent.name, arizona);
		assert_eq!(most_recent.outer.unwrap().name, usa);
		// }}}

		// {{{
		let (desert_view, earth) = futures::try_join!(
			PgLocation::create(&connection, "Desert View".into(), None),
			PgLocation::create(&connection, "Earth".into(), None),
		)
		.unwrap();

		let filepath = utils::temp_file::<Create>("run-action");
		utils::write_yaml(&filepath, MatchLocation::from(earth.id));

		let filepath2 = filepath.with_file_name("run-action-2");
		let match_desert_view = MatchLocation::from(desert_view.id);
		utils::write_yaml(&filepath2, &match_desert_view);

		run(config.clone(), CreateCommand::Location {
			inside: FlagOrArgument::Argument(filepath.clone()),
			names,
			outside: FlagOrArgument::Argument(filepath2),
		})
		.await;

		let db_desert_view = PgLocation::retrieve(&connection, match_desert_view)
			.await
			.map(|mut v| v.remove(0))
			.unwrap();

		assert_eq!(db_desert_view.id, desert_view.id);
		assert_eq!(db_desert_view.name, desert_view.name);

		let db_arizona = db_desert_view.outer.unwrap();
		assert_eq!(db_arizona.name, arizona);

		let db_usa = db_arizona.outer.unwrap();
		assert_eq!(db_usa.name, usa);
		assert_eq!(db_usa.outer.as_deref().unwrap(), &earth);
		// }}}

		/* ########## `clinvoice create contact` ########## */

		let location_id = most_recent.id;

		// {{{
		let info = "@my_username";
		let label = "Superunique Twitter Handle";

		run(config.clone(), CreateCommand::Contact {
			address: FlagOrArgument::Flag(false),
			email: false,
			info: info.into(),
			label: label.into(),
			phone: false,
		})
		.await;

		let most_recent = contact(&connection, label).await.unwrap();

		assert_eq!(most_recent.label, label);
		assert_eq!(most_recent.kind.other(), Some(info));
		// }}}

		utils::write_yaml(&filepath, MatchLocation::from(location_id));
		let label = "Superunique Locationnn Handle";

		run(config.clone(), CreateCommand::Contact {
			address: FlagOrArgument::Argument(filepath.clone()),
			email: false,
			info: String::new(),
			label: label.into(),
			phone: false,
		})
		.await;

		let most_recent2 = contact(&connection, label).await.unwrap();

		assert_eq!(most_recent2.label, label);
		assert_eq!(most_recent2.kind.address().map(|a| a.id), Some(location_id));
		// }}}

		// {{{
		let label = "Superunique Emailsdlkj Handle";
		let info = "my_address@company.org";

		run(config.clone(), CreateCommand::Contact {
			address: FlagOrArgument::Flag(false),
			email: true,
			info: info.into(),
			label: label.into(),
			phone: false,
		})
		.await;

		let most_recent3 = contact(&connection, label).await.unwrap();

		assert_eq!(most_recent3.label, label);
		assert_eq!(most_recent3.kind.email(), Some(info));
		// }}}

		// {{{
		let label = "Superunique Phoneadlskjfh Handle";
		let info = "18005555555";

		run(config.clone(), CreateCommand::Contact {
			address: FlagOrArgument::Flag(false),
			email: false,
			info: info.into(),
			label: label.into(),
			phone: true,
		})
		.await;

		let most_recent4 = contact(&connection, label).await.unwrap();

		assert_eq!(most_recent4.label, label);
		assert_eq!(most_recent4.kind.phone(), Some(info));
		// }}}

		// created contacts must be cleaned up
		PgContact::delete(
			&connection,
			[most_recent, most_recent2, most_recent3, most_recent4].iter(),
		)
		.await
		.unwrap();

		/* ########## `clinvoice create organization` ########## */

		// {{{
		let name = "Foo";

		run(config.clone(), CreateCommand::Organization {
			location: Some(filepath.clone()),
			name: name.into(),
		})
		.await;

		let most_recent =
			latest_entity::<PgOrganization, OrganizationColumns<&str>>(&connection).await.unwrap();

		assert_eq!(most_recent.location.id, location_id);
		assert_eq!(most_recent.name, name);
		// }}}

		config.organizations.employer_id = Some(most_recent.id);

		/* ########## `clinvoice create job` ########## */

		// {{{
		let invoice = Invoice { hourly_rate: Money::new(17_60, 2, Currency::Usd), date: None };
		let notes = "Placeholder";
		let objectives = "Test `clinvoice create job --employer`";

		run(config.clone(), CreateCommand::Job {
			client: None,
			date_close: None,
			date_invoice_issued: None,
			date_invoice_paid: None,
			date_open: None,
			employer: true,
			hourly_rate: invoice.hourly_rate,
			increment: None,
			notes: notes.into(),
			objectives: objectives.into(),
		})
		.await;

		let most_recent = latest_entity::<PgJob, JobColumns<&str>>(&connection).await.unwrap();
		let exchange_rates = exchange_rates_fut.await.unwrap();

		assert_eq!(most_recent.client.id, config.organizations.employer_id.unwrap());
		assert_eq!(most_recent.date_close, None);
		assert_eq!(most_recent.invoice.exchange(Currency::Usd, &exchange_rates), invoice);
		assert_eq!(most_recent.increment, config.jobs.default_increment);
		assert_eq!(most_recent.notes, notes);
		assert_eq!(most_recent.objectives, objectives);
		// }}}

		// {{{
		let date_open = NaiveDate::from_ymd(2020, 06, 20).and_hms(04, 05, 00);
		let date_close = date_open + Duration::days(1);
		let invoice = Invoice {
			date: Some(InvoiceDate {
				issued: utils::naive_local_datetime_to_utc(date_close + Duration::days(1)),
				paid: Some(utils::naive_local_datetime_to_utc(date_close + Duration::days(2))),
			}),
			..invoice
		};

		utils::write_yaml(
			&filepath,
			config.organizations.employer_id.map(MatchOrganization::from).unwrap(),
		);

		run(config.clone(), {
			CreateCommand::Job {
				client: Some(filepath.clone()),
				date_close: Some(date_close),
				date_invoice_issued: invoice
					.date
					.map(|d| DateTime::<Local>::from(d.issued).naive_local()),
				date_invoice_paid: invoice
					.date
					.and_then(|d| d.paid)
					.map(|paid| DateTime::<Local>::from(paid).naive_local()),
				date_open: Some(date_open),
				employer: false,
				hourly_rate: invoice.hourly_rate,
				increment: Some(config.jobs.default_increment),
				notes: notes.into(),
				objectives: objectives.into(),
			}
		})
		.await;

		let most_recent = latest_entity::<PgJob, JobColumns<&str>>(&connection).await.unwrap();

		assert_eq!(most_recent.client.id, config.organizations.employer_id.unwrap());
		assert_eq!(most_recent.date_open, utils::naive_local_datetime_to_utc(date_open));
		assert_eq!(most_recent.date_close, Some(utils::naive_local_datetime_to_utc(date_close)));
		assert_eq!(most_recent.invoice.exchange(Currency::Usd, &exchange_rates), invoice);
		assert_eq!(most_recent.increment, config.jobs.default_increment);
		assert_eq!(most_recent.notes, notes);
		assert_eq!(most_recent.objectives, objectives);
		// }}}

		/* ########## `clinvoice create timesheet` ########## */

		let job_id = most_recent.id;

		// {{{
		let time_begin = Utc::now();
		utils::write_yaml(&filepath, MatchJob::from(job_id));

		run(config.clone(), CreateCommand::Timesheet {
			default_employee: true,
			employee: None,
			job: Some(filepath.clone()),
			time_begin: None,
			time_end: None,
			work_notes: None,
		})
		.await;

		let most_recent =
			latest_entity::<PgTimesheet, TimesheetColumns<&str>>(&connection).await.unwrap();

		assert_eq!(most_recent.employee.id, config.employees.id.unwrap());
		assert!(most_recent.expenses.is_empty());
		assert_eq!(most_recent.job.id, job_id);

		// NOTE: we can't know what the exact begin time was, since `time_begin: None` should set
		//       `time_begin` to `Utc::now()` at some point between RIGHT NOW and before `run` was
		//       called.
		assert!(most_recent.time_begin > time_begin);
		assert!(most_recent.time_begin < Utc::now());

		assert_eq!(most_recent.time_end, None);
		assert!(most_recent.work_notes.is_empty());
		// }}}

		// {{{
		let time_end = time_begin + Duration::days(1);
		let work_notes = "These are my notes";
		let filepath2 = filepath.with_file_name("run-action-2");
		utils::write_yaml(&filepath2, config.employees.id.map(MatchEmployee::from).unwrap());

		run(config.clone(), CreateCommand::Timesheet {
			default_employee: false,
			employee: Some(filepath2),
			job: Some(filepath.clone()),
			time_begin: Some(DateTime::<Local>::from(time_begin).naive_local()),
			time_end: Some(DateTime::<Local>::from(time_end).naive_local()),
			work_notes: Some(work_notes.into()),
		})
		.await;

		let most_recent =
			latest_entity::<PgTimesheet, TimesheetColumns<&str>>(&connection).await.unwrap();

		assert_eq!(most_recent.employee.id, config.employees.id.unwrap());
		assert!(most_recent.expenses.is_empty());
		assert_eq!(most_recent.job.id, job_id);
		assert_eq!(
			most_recent.time_begin,
			utils::naive_local_datetime_to_utc(DateTime::<Local>::from(time_begin).naive_local())
		);
		assert_eq!(
			most_recent.time_end,
			Some(utils::naive_local_datetime_to_utc(
				DateTime::<Local>::from(time_end).naive_local()
			))
		);
		assert_eq!(most_recent.work_notes, work_notes);
		// }}}

		/* ########## `clinvoice create expense` ########## */

		let timesheet_id = most_recent.id;

		// {{{
		let category = "Food";
		let cost = Money::new(53_42, 2, Currency::Usd);
		let description = "Two number nines,
			a number nine large,
			a number six with extra dip,
			two number fourty-fives (one with cheese),
			and a large soda.";

		utils::write_yaml(&filepath, MatchTimesheet::from(timesheet_id));

		run(config, CreateCommand::Expense {
			category: category.into(),
			cost,
			description: description.into(),
			timesheet: Some(filepath),
		})
		.await;

		let most_recent =
			latest_entity::<PgExpenses, ExpenseColumns<&str>>(&connection).await.unwrap();

		assert_eq!(most_recent.category, category);
		assert_eq!(most_recent.cost.exchange(Currency::Usd, &exchange_rates), cost);
		assert_eq!(most_recent.description, description);
		assert_eq!(most_recent.timesheet_id, timesheet_id);
		// }}}
	}
}

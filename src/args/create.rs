mod command;

use clap::Args as Clap;
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
use clinvoice_config::{Adapters, Config, Error as ConfigError};
use clinvoice_match::{MatchEmployee, MatchOrganization};
use clinvoice_schema::{chrono::Utc, ContactKind, Invoice, InvoiceDate};
use command::CreateCommand;
use sqlx::{Database, Executor, Pool, Transaction};

use super::store_args::StoreArgs;
use crate::{
	args::update::Update,
	input,
	utils::{self, Identifiable},
	DynResult,
};

/// Use CLInvoice to store new information.
///
/// CLInvoice is capable of storing multiple kinds of information. This command has multiple
/// subcommands and options which will guide you through the process and ensure that the data
/// provided is valid.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Create
{
	/// The object to [`Create`] and related arguments.
	#[clap(subcommand)]
	command: CreateCommand,

	/// Specifies the [`Store`](clinvoice_config::Store) to insert [`Create`]d data into.
	#[clap(flatten)]
	store_args: StoreArgs,
}

impl Create
{
	/// [`Create`] an entity according to the [`CreateCommand`].
	///
	/// The [`StoreArgs`] must be resolved into a `connection` by this point.
	async fn create<CAdapter, EAdapter, JAdapter, LAdapter, OAdapter, TAdapter, XAdapter, TDb>(
		self,
		connection: Pool<TDb>,
		config: &Config,
	) -> DynResult<()>
	where
		TDb: Database,
		CAdapter: Deletable<Db = TDb> + ContactAdapter,
		EAdapter: Deletable<Db = TDb> + EmployeeAdapter,
		JAdapter: Deletable<Db = TDb> + JobAdapter,
		LAdapter: Deletable<Db = TDb> + LocationAdapter,
		OAdapter: Deletable<Db = TDb> + OrganizationAdapter,
		TAdapter: Deletable<Db = TDb> + TimesheetAdapter,
		XAdapter: Deletable<Db = TDb> + ExpensesAdapter,
		for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
		for<'c> &'c mut Transaction<'c, TDb>: Executor<'c, Database = TDb>,
	{
		match self.command
		{
			CreateCommand::Contact {
				label,
				address,
				email,
				phone,
				info,
			} =>
			{
				let kind = match (address, email, phone)
				{
					(true, ..) => input::select_one_retrieved::<LAdapter, _, _>(
						&connection,
						None,
						"Query the Location of this address",
					)
					.await
					.map(ContactKind::Address)?,
					(_, true, _) => ContactKind::Email(info),
					(.., true) => ContactKind::Phone(info),
					_ => ContactKind::Other(info),
				};

				let created = CAdapter::create(&connection, kind, label).await?;
				Self::report_created(&created);
			},

			CreateCommand::Employee {
				name,
				status,
				title,
			} =>
			{
				let created = EAdapter::create(&connection, name, status, title).await?;
				Self::report_created(&created);
			},

			CreateCommand::Expense {
				category,
				cost,
				description,
			} =>
			{
				let timesheet = input::select_one_retrieved::<TAdapter, _, _>(
					&connection,
					None,
					"Query the Timesheet this Expense is for",
				)
				.await?;

				let created = XAdapter::create(
					&connection,
					vec![(category, cost, description)],
					timesheet.id,
				)
				.await
				.map(|mut v| {
					v.pop()
						.expect("at least one `Expense` should have been created")
				})?;

				Self::report_created(&created);
			},

			CreateCommand::Job {
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
				let match_condition = employer
					.then(|| {
						config
							.organizations
							.employer_id
							.map(MatchOrganization::from)
							.ok_or(
								"The `employer_id` key in the `[organizations]` field of the \
								 configuration file has no value",
							)
					})
					.transpose()?;

				let client = input::select_one_retrieved::<OAdapter, _, _>(
					&connection,
					match_condition,
					"Query the client Organization for this Job",
				)
				.await?;

				let created = JAdapter::create(
					&connection,
					client,
					date_close.map(utils::naive_local_datetime_to_utc),
					date_open
						.map(utils::naive_local_datetime_to_utc)
						.unwrap_or_else(|| Utc::now()),
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

			CreateCommand::Location {
				inside,
				outside,
				names,
			} =>
			{
				let mut names_reversed = names.into_iter().rev();

				let final_name = names_reversed
					.next()
					.expect("clap config should have ensured that `names` has length of at least one");

				let outside_of_final = match inside
				{
					true => input::select_one_retrieved::<LAdapter, _, _>(
						&connection,
						None,
						format!("Query the Location outside of {final_name}"),
					)
					.await
					.map(Some)?,
					_ => None,
				};

				// {{{
				let mut transaction = connection.begin().await?;

				// TODO: convert to `try_fold` after `stream`s merge to `std`? {{{2
				let mut l = LAdapter::create(&mut *transaction, final_name, outside_of_final).await?;
				Self::report_created(&l);
				for n in names_reversed
				{
					l = LAdapter::create(&mut *transaction, n, Some(l)).await?;
					Self::report_created(&l);
				}
				// 2}}}

				let created = l;

				if outside
				{
					let mut inside_locations = input::select_retrieved::<LAdapter, _, _>(
						&connection,
						None,
						format!("Select Locations that are inside {created}"),
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

					LAdapter::update(
						&mut transaction,
						inside_locations.iter().map(|l| {
							// HACK: can't pass `readable` in directly
							Update::report_updated(l);
							l
						}),
					)
					.await?;
				}

				transaction.commit().await?;
				// }}}
			},

			CreateCommand::Organization { name } =>
			{
				let selected = input::select_one_retrieved::<LAdapter, _, _>(
					&connection,
					None,
					"Query the Location of this Organization",
				)
				.await?;

				let created = OAdapter::create(&connection, selected, name).await?;
				Self::report_created(&created);
			},

			CreateCommand::Timesheet {
				default_employee,
				time_begin,
				time_end,
				work_notes,
			} =>
			{
				let match_condition = default_employee
					.then(|| {
						config.employees.id.map(MatchEmployee::from).ok_or(
							"The `id` key in the `[employees]` field of the configuration file has no \
							 value",
						)
					})
					.transpose()?;

				let employee = input::select_one_retrieved::<EAdapter, _, _>(
					&connection,
					match_condition,
					"Query the Employee who is responsible for the work",
				)
				.await?;

				let job = input::select_one_retrieved::<JAdapter, _, _>(
					&connection,
					None,
					"Query the Job being worked on",
				)
				.await?;

				let expenses = input::expense::menu()?;

				// {{{
				let mut transaction = connection.begin().await?;

				let created = TAdapter::create(
					&mut transaction,
					employee,
					expenses,
					job,
					time_begin
						.map(utils::naive_local_datetime_to_utc)
						.unwrap_or_else(|| Utc::now()),
					time_end.map(utils::naive_local_datetime_to_utc),
					work_notes.unwrap_or_else(|| "None".into()),
				)
				.await?;

				transaction.commit().await?;
				// }}}

				Self::report_created(&created);
			},
		};

		Ok(())
	}

	/// Indicate with [`println!`] that a value of type `TCreated` — [`Display`]ed by calling
	/// `selector` on the `created` value — was created.
	pub(super) fn report_created<TCreated>(created: &TCreated)
	where
		TCreated: Identifiable,
	{
		utils::report_action("created", created);
	}

	/// Execute this command given the user's [`Config`].
	pub async fn run(self, config: &Config) -> DynResult<()>
	{
		let store = self.store_args.try_get_from(config)?;

		match store.adapter
		{
			#[cfg(feature = "postgres")]
			Adapters::Postgres =>
			{
				use clinvoice_adapter_postgres::schema::{
					PgContact,
					PgEmployee,
					PgExpenses,
					PgJob,
					PgLocation,
					PgOrganization,
					PgTimesheet,
				};

				let pool = Pool::connect_lazy(&store.url)?;
				self
					.create::<PgContact, PgEmployee, PgJob, PgLocation, PgOrganization, PgTimesheet, PgExpenses, _>(
						pool, config,
					)
					.await?
			},

			// NOTE: this is allowed because there may be additional adapters added later, and I want
			//       to define this behavior now.
			#[allow(unreachable_patterns)]
			_ => return Err(ConfigError::FeatureNotFound(store.adapter).into()),
		};

		Ok(())
	}
}

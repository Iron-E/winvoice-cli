mod command;

use core::fmt::Display;

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
use clinvoice_match::{Match, MatchOrganization};
use clinvoice_schema::{
	chrono::Utc,
	Contact,
	ContactKind,
	Employee,
	Invoice,
	InvoiceDate,
	Job,
	Location,
	Organization,
};
use command::CreateCommand;
use futures::{TryFutureExt, TryStreamExt};
use sqlx::{Database, Executor, Pool, Transaction};

use super::store_args::StoreArgs;
use crate::{args::update::Update, fmt, input, utils, DynError, DynResult};

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
	pub async fn create<CAdapter, EAdapter, JAdapter, LAdapter, OAdapter, TAdapter, XAdapter, TDb>(
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
					(true, ..) => input::select_one_retrieved::<LAdapter, _, _, true>(
						&connection,
						"Query the Location of this address",
					)
					.await
					.map(ContactKind::Address)?,
					(_, true, _) => ContactKind::Email(info),
					(.., true) => ContactKind::Phone(info),
					_ => ContactKind::Other(info),
				};

				Self::report_created::<Contact, _>(fmt::quoted(
					CAdapter::create(&connection, kind, label).await?.label,
				));
			},

			CreateCommand::Employee {
				name,
				status,
				title,
			} =>
			{
				Self::report_created::<Employee, _>(fmt::id_num(
					EAdapter::create(&connection, name, status, title).await?.id,
				));
			},

			CreateCommand::Expense {
				category,
				cost,
				description,
			} => todo!(),

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
				let client = match employer
				{
					true =>
					{
						let mut retrieved =
							OAdapter::retrieve(&connection, &MatchOrganization {
								id: config.organizations.employer_id.map(Match::from).ok_or(
									"The `employer_id` field of the configuration file was not set",
								)?,
								..Default::default()
							})
							.await?;

						retrieved
							.pop()
							.ok_or_else(|| input::Error::NoData(fmt::type_name::<Organization>().into()))?
					},

					#[rustfmt::skip]
					_ => input::select_one_retrieved::<OAdapter, _, _, true>(
						&connection,
						"Query the client for this Job",
					)
					.await?,
				};

				Self::report_created::<Job, _>(
					JAdapter::create(
						&connection,
						client,
						date_close.map(utils::naive_local_datetime_to_utc),
						date_open
							.map(utils::naive_local_datetime_to_utc)
							.unwrap_or_else(|| Utc::now()),
						increment,
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
					.await?
					.id,
				);
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
					true => input::select_one_retrieved::<LAdapter, _, _, true>(
						&connection,
						format!("Query the Location outside of {final_name}"),
					)
					.await
					.map(Some)?,
					_ => None,
				};

				// {{{
				let mut transaction = connection.begin().await?;

				/// A human-readable version of `Location`.
				fn readable(l: &Location) -> String
				{
					format!("{} {}", fmt::id_num(l.id), fmt::quoted(&l.name))
				}

				// TODO: convert to `try_fold` after `stream`s merge to `std`? {{{2
				let mut l = LAdapter::create(&mut *transaction, final_name, outside_of_final).await?;
				Self::report_created::<Location, _>(readable(&l));
				for n in names_reversed
				{
					l = LAdapter::create(&mut *transaction, n, Some(l)).await?;
					Self::report_created::<Location, _>(readable(&l));
				}
				// 2}}}

				let created = l;

				if outside
				{
					let mut inside_locations = input::select_retrieved::<LAdapter, _, _, true>(
						&connection,
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
							Update::report_updated::<Location, _>(readable(&l));
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
				let selected = input::select_one_retrieved::<LAdapter, _, _, true>(
					&connection,
					"Query the Location of this Organization",
				)
				.await?;

				Self::report_created::<Organization, _>(fmt::id_num(
					OAdapter::create(&connection, selected, name).await?.id,
				));
			},

			CreateCommand::Timesheet {
				default_employee,
				time_begin,
				time_end,
				work_notes,
			} => todo!(),
		};

		Ok(())
	}

	/// Indicate with [`println!`] that a value of type `TCreated` — identified by `id` — has been
	/// created successfully.
	pub(super) fn report_created<TCreated, TId>(id: TId)
	where
		TId: Display,
	{
		println!("{} {id} has been created.", fmt::type_name::<TCreated>());
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
				use sqlx::PgPool;

				let pool = PgPool::connect_lazy(&store.url)?;
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

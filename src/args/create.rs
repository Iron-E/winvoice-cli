mod command;

use clap::Args as Clap;
use clinvoice_adapter::{
	schema::{ContactInfoAdapter, EmployeeAdapter, LocationAdapter, OrganizationAdapter},
	Deletable,
};
use clinvoice_config::{Adapters, Config, Error as ConfigError};
use clinvoice_schema::{Contact, ContactKind};
use command::CreateCommand;
use sqlx::{Database, Executor, Pool};

use super::store_args::StoreArgs;
use crate::{input, DynResult};

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
	pub async fn create<Db, CAdapter, EAdapter, LAdapter, OAdapter>(
		self,
		connection: Pool<Db>,
		config: &Config,
	) -> DynResult<()>
	where
		Db: Database,
		CAdapter: Deletable<Db = Db> + ContactInfoAdapter,
		EAdapter: Deletable<Db = Db> + EmployeeAdapter,
		LAdapter: Deletable<Db = Db> + LocationAdapter,
		OAdapter: Deletable<Db = Db> + OrganizationAdapter,
		for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
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
					(true, ..) => input::util::location::select_one::<_, _, LAdapter, true>(
						&connection,
						"Query the `Location` of this address",
					)
					.await
					.map(ContactKind::Address)?,
					(_, true, _) => ContactKind::Email(info),
					(.., true) => ContactKind::Phone(info),
					_ => ContactKind::Other(info),
				};

				CAdapter::create(&connection, [Contact { label, kind }].iter()).await?;
			},

			CreateCommand::Employee {
				name,
				status,
				title,
			} =>
			{
				EAdapter::create(&connection, name, status, title).await?;
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
				hourly_rate,
				increment,
				notes,
				objectives,
			} => todo!(),

			CreateCommand::Location {
				inside,
				outside,
				names,
			} => todo!(),

			CreateCommand::Organization { name } =>
			{
				let selected = input::util::location::select_one::<_, _, LAdapter, true>(
					&connection,
					"Query the `Location` of this `Organization`",
				)
				.await?;

				OAdapter::create(&connection, selected, name).await?;
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

	pub async fn run(self, config: &Config) -> DynResult<()>
	{
		let store = self.store_args.try_get_from(config)?;

		match store.adapter
		{
			#[cfg(feature = "postgres")]
			Adapters::Postgres =>
			{
				use clinvoice_adapter_postgres::schema::{
					PgContactInfo,
					PgEmployee,
					PgLocation,
					PgOrganization,
				};
				use sqlx::PgPool;

				let pool = sqlx::PgPool::connect_lazy(&store.url)?;
				self
					.create::<_, PgContactInfo, PgEmployee, PgLocation, PgOrganization>(pool, config)
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

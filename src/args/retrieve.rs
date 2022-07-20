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
	Retrievable,
};
use clinvoice_config::{Adapters, Config, Error as ConfigError};
use command::RetrieveCommand;
use serde::{de::DeserializeOwned, Serialize};
use sqlx::{Database, Executor, Pool};

use super::{match_args::MatchArgs, store_args::StoreArgs};
use crate::{fmt, input, DynResult};

/// Retrieve information being stored by CLInvoice.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Retrieve
{
	/// The specific object to [`Retrieve`] and related arguments.
	#[clap(subcommand)]
	command: RetrieveCommand,

	/// Specifies a file which can be used in place of the prompt of a user query.
	#[clap(flatten)]
	match_args: MatchArgs,

	/// Specifies the [`Store`](clinvoice_config::Store) to [`Retrieve`] from.
	#[clap(flatten)]
	store_args: StoreArgs,
}

impl Retrieve
{
	/// [`Retrieve`] an entity according to the [`RetrieveCommand`].
	///
	/// The [`StoreArgs`] must be resolved into a `connection` by this point.
	async fn retrieve<CAdapter, EAdapter, JAdapter, LAdapter, OAdapter, TAdapter, XAdapter, TDb>(
		self,
		connection: Pool<TDb>,
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
	{
		/// A generic deletion function which works for any of the provided adapters in the outer
		/// function, as they all implement `TDelRetrievable` at the minimum.
		async fn retrieve<TRetrievable, TDb>(
			connection: Pool<TDb>,
			match_args: MatchArgs,
		) -> DynResult<Vec<TRetrievable::Entity>>
		where
			TDb: Database,
			TRetrievable: Retrievable<Db = TDb>,
			TRetrievable::Entity: Clone + Display + Sync,
			TRetrievable::Match: Default + DeserializeOwned + Serialize,
			for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
		{
			Ok(match match_args.deserialize()?
			{
				Some(condition) => TRetrievable::retrieve(&connection, &condition).await?,

				#[rustfmt::skip]
				_ => input::retrieve::<TRetrievable, _, _>(
					&connection,
					format!("Query the {} to delete", fmt::type_name::<TRetrievable::Entity>()),
				)
				.await?,
			})
		}

		match self.command
		{
			RetrieveCommand::Contact =>
			{
				let retrieved = retrieve::<CAdapter, _>(connection, self.match_args).await?;
			},

			RetrieveCommand::Employee {
				default,
				set_default,
			} =>
			{
				let retrieved = retrieve::<EAdapter, _>(connection, self.match_args).await?;
			},

			RetrieveCommand::Expense =>
			{
				let retrieved = retrieve::<XAdapter, _>(connection, self.match_args).await?;
			},

			RetrieveCommand::Job {
				export,
				format,
				output_dir,
			} =>
			{
				let retrieved = retrieve::<JAdapter, _>(connection, self.match_args).await?;
			},

			RetrieveCommand::Location =>
			{
				let retrieved = retrieve::<LAdapter, _>(connection, self.match_args).await?;
			},

			RetrieveCommand::Organization {
				employer,
				set_employer,
			} =>
			{
				let retrieved = retrieve::<OAdapter, _>(connection, self.match_args).await?;
			},

			RetrieveCommand::Timesheet =>
			{
				let retrieved = retrieve::<TAdapter, _>(connection, self.match_args).await?;
			},
		};

		Ok(())
	}

	pub async fn run(self, config: Config) -> DynResult<()>
	{
		let store = self.store_args.try_get_from(&config)?;

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
					.retrieve::<PgContact, PgEmployee, PgJob, PgLocation, PgOrganization, PgTimesheet, PgExpenses, _>(
						pool,
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

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
use command::DeleteCommand;
use serde::{de::DeserializeOwned, Serialize};
use sqlx::{Database, Executor, Pool};

use super::{match_args::MatchArgs, store_args::StoreArgs};
use crate::{fmt, input, DynResult};

/// Delete data which is being stored by CLInvoice.
///
/// CLInvoice stores data which references other data. For example, an `Organization` exists in a
/// `Location`. So, if you attempt to delete any information which is being referenced by other
/// information (e.g. the `Location` of an `Organization`), this operation will fail.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Delete
{
	/// The specifies the object to [`Delete`] and related arguments.
	#[clap(subcommand)]
	command: DeleteCommand,

	/// Specifies a file which can be used in place of the prompt of a user query.
	#[clap(flatten)]
	match_args: MatchArgs,

	/// Specifies the [`Store`](clinvoice_config::Store) to [`Delete`] from.
	#[clap(flatten)]
	store_args: StoreArgs,
}

impl Delete
{
	/// [`Delete`] an entity according to the [`DeleteCommand`].
	///
	/// The [`StoreArgs`] must be resolved into a `connection` by this point.
	async fn delete<CAdapter, EAdapter, JAdapter, LAdapter, OAdapter, TAdapter, XAdapter, TDb>(
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
		/// function, as they all implement `TDelRetr` at the minimum.
		async fn del<TDelRetrievable, TDb>(connection: Pool<TDb>) -> DynResult<()>
		where
			TDb: Database,
			TDelRetrievable: Deletable<Db = TDb>,
			<TDelRetrievable as Deletable>::Entity: Clone + Display + Sync,
			TDelRetrievable: Retrievable<Db = TDb, Entity = <TDelRetrievable as Deletable>::Entity>,
			<TDelRetrievable as Retrievable>::Match: Default + DeserializeOwned + Serialize,
			for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
		{
			let selected = input::select_retrieved::<TDelRetrievable, _, _>(
				&connection,
				format!(
					"Query the {} to delete",
					fmt::type_name::<<TDelRetrievable as Deletable>::Entity>()
				),
			)
			.await?;

			TDelRetrievable::delete(&connection, selected.iter()).await?;
			Ok(())
		}

		match self.command
		{
			DeleteCommand::Contact => del::<CAdapter, _>(connection).await?,
			DeleteCommand::Employee => del::<EAdapter, _>(connection).await?,
			DeleteCommand::Expense => del::<XAdapter, _>(connection).await?,
			DeleteCommand::Job => del::<JAdapter, _>(connection).await?,
			DeleteCommand::Location => del::<LAdapter, _>(connection).await?,
			DeleteCommand::Organization => del::<OAdapter, _>(connection).await?,
			DeleteCommand::Timesheet => del::<TAdapter, _>(connection).await?,
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
					.delete::<PgContact, PgEmployee, PgJob, PgLocation, PgOrganization, PgTimesheet, PgExpenses, _>(
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

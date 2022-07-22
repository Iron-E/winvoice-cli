mod command;

use core::fmt::Display;
use std::error::Error;

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
use crate::{
	fmt,
	input,
	utils::{self, Identifiable},
	DynResult,
};

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
		/// function, as they all implement `TDelRetrievable` at the minimum.
		async fn del<TDelRetrievable, TDb, TMatch>(
			connection: Pool<TDb>,
			match_condition: TMatch,
		) -> DynResult<()>
		where
			TDb: Database,
			TMatch: TryInto<Option<TDelRetrievable::Match>>,
			TMatch::Error: 'static + Error,
			TDelRetrievable: Deletable<Db = TDb>,
			<TDelRetrievable as Deletable>::Entity: Clone + Display + Identifiable + Sync,
			TDelRetrievable: Retrievable<Db = TDb, Entity = <TDelRetrievable as Deletable>::Entity>,
			TDelRetrievable::Match: Default + DeserializeOwned + Serialize,
			for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
		{
			let match_condition = match_condition.try_into()?;
			let type_name = fmt::type_name::<<TDelRetrievable as Deletable>::Entity>();
			let retrieved = input::select_retrieved::<TDelRetrievable, _, _>(
				&connection,
				match_condition,
				format!("Query the {type_name} to delete"),
			)
			.await?;

			let selected = input::select(&retrieved, format!("Select the {type_name} to delete"))?;
			TDelRetrievable::delete(
				&connection,
				selected.iter().map(|s| {
					Delete::report_deleted(s);
					s
				}),
			)
			.await?;
			Ok(())
		}

		match self.command
		{
			DeleteCommand::Contact => del::<CAdapter, _, _>(connection, self.match_args).await,
			DeleteCommand::Employee => del::<EAdapter, _, _>(connection, self.match_args).await,
			DeleteCommand::Expense => del::<XAdapter, _, _>(connection, self.match_args).await,
			DeleteCommand::Job => del::<JAdapter, _, _>(connection, self.match_args).await,
			DeleteCommand::Location => del::<LAdapter, _, _>(connection, self.match_args).await,
			DeleteCommand::Organization => del::<OAdapter, _, _>(connection, self.match_args).await,
			DeleteCommand::Timesheet => del::<TAdapter, _, _>(connection, self.match_args).await,
		}
	}

	/// Indicate with [`println!`] that a value of type `TDeleted` — [`Display`]ed by calling
	/// `selector` on the `deleted` value — was deleted.
	pub(super) fn report_deleted<TDeleted>(deleted: &TDeleted)
	where
		TDeleted: Identifiable,
	{
		utils::report_action("deleted", deleted);
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

				let pool = Pool::connect_lazy(&store.url)?;
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

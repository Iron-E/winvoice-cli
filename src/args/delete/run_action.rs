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
	async fn action<CAdapter, EAdapter, JAdapter, LAdapter, OAdapter, TAdapter, XAdapter, TDb>(
		self,
		connection: Pool<TDb>,
		_config: Config,
	) -> DynResult<()>
	where
		CAdapter: Deletable<Db = TDb> + ContactAdapter,
		EAdapter: Deletable<Db = TDb> + EmployeeAdapter,
		JAdapter: Deletable<Db = TDb> + JobAdapter,
		LAdapter: Deletable<Db = TDb> + LocationAdapter,
		OAdapter: Deletable<Db = TDb> + OrganizationAdapter,
		TAdapter: Deletable<Db = TDb> + TimesheetAdapter,
		XAdapter: Deletable<Db = TDb> + ExpensesAdapter,
		TDb: Database,
		for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
	{
		/// A generic deletion function which works for any of the provided adapters in the outer
		/// function, as they all implement `TDelRetrievable` at the minimum.
		async fn del<TDelRetrievable, TDb, TMatch>(
			connection: &Pool<TDb>,
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
				connection,
				match_condition,
				format!("Query the {type_name} to delete"),
			)
			.await?;

			let selected = input::select(&retrieved, format!("Select the {type_name} to delete"))?;
			TDelRetrievable::delete(
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
			DeleteCommand::Organization => del::<OAdapter, _, _>(&connection, self.match_args).await,
			DeleteCommand::Timesheet => del::<TAdapter, _, _>(&connection, self.match_args).await,
		}
	}
}

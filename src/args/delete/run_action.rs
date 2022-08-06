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

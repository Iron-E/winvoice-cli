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
use clinvoice_config::{Adapters, Config, Error};
use sqlx::{Database, Executor, Pool, Transaction};

use super::store_args::StoreArgs;
use crate::DynResult;

#[async_trait::async_trait(?Send)]
pub trait RunAction: AsRef<StoreArgs> + Sized
{
	/// Perform this command's action using a specific set of database-struct adapters.
	async fn action<CAdapter, EAdapter, JAdapter, LAdapter, OAdapter, TAdapter, XAdapter, TDb>(
		self,
		connection: Pool<TDb>,
		config: Config,
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
		for<'c> &'c mut Transaction<'c, TDb>: Executor<'c, Database = TDb>;

	/// Execute this command given the user's [`Config`].
	async fn run(self, config: Config) -> DynResult<()>
	{
		let store = self.as_ref().try_get_from(&config)?;

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
					.action::<PgContact, PgEmployee, PgJob, PgLocation, PgOrganization, PgTimesheet, PgExpenses, _>(
						pool, config,
					)
					.await?
			},

			// NOTE: this is allowed because there may be additional adapters added later, and I want
			//       to define this behavior now.
			#[allow(unreachable_patterns)]
			_ => return Err(Error::FeatureNotFound(store.adapter).into()),
		};

		Ok(())
	}
}
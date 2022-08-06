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
	async fn action<CAdapter, EAdapter, JAdapter, LAdapter, OAdapter, TAdapter, XAdapter, Db>(
		self,
		connection: Pool<Db>,
		config: Config,
	) -> DynResult<()>
	where
		Db: Database,
		CAdapter: Deletable<Db = Db> + ContactAdapter,
		EAdapter: Deletable<Db = Db> + EmployeeAdapter,
		JAdapter: Deletable<Db = Db> + JobAdapter,
		LAdapter: Deletable<Db = Db> + LocationAdapter,
		OAdapter: Deletable<Db = Db> + OrganizationAdapter,
		TAdapter: Deletable<Db = Db> + TimesheetAdapter,
		XAdapter: Deletable<Db = Db> + ExpensesAdapter,
		for<'connection> &'connection mut Db::Connection: Executor<'connection, Database = Db>,
		for<'connection> &'connection mut Transaction<'connection, Db>:
			Executor<'connection, Database = Db>;

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

			// NOTE: this is allowed because there may be additional adapters added later, and I
			// want       to define this behavior now.
			#[allow(unreachable_patterns)]
			_ => return Err(Error::FeatureNotFound(store.adapter).into()),
		};

		Ok(())
	}
}

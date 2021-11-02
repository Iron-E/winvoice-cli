mod command;

use clinvoice_adapter::{Adapters, Error as AdapterError, Store};
use clinvoice_adapter_postgres::schema::PostgresTimesheet;
use clinvoice_config::Config;
use command::Command;
use structopt::StructOpt;
#[cfg(feature = "postgres")]
use {
	clinvoice_adapter_postgres::schema::{
		PostgresEmployee,
		PostgresJob,
		PostgresLocation,
		PostgresOrganization,
		PostgresPerson,
	},
	sqlx::PgPool,
};

use crate::DynResult;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(about = "Retrieve information that was recorded with CLInvoice")]
pub struct Retrieve
{
	#[structopt(help = "Select retrieved entities for deletion. See -c", long, short)]
	delete: bool,

	#[structopt(
		help = "Cascade -d operations. Without this flag, entities referenced by other entities \
		        cannot be deleted",
		long = "cascade",
		short = "c"
	)]
	cascade_delete: bool,

	#[structopt(help = "Select retrieved entities for data updating", long, short)]
	update: bool,

	#[structopt(subcommand)]
	command: Command,
}

impl Retrieve
{
	/// # Summary
	///
	/// Execute the constructed command.
	pub async fn run<'err>(self, config: &Config<'_, '_>, store: &Store) -> DynResult<'err, ()>
	{
		match store.adapter
		{
			#[cfg(feature = "postgres")]
			Adapters::Postgres =>
			{
				let pool = PgPool::connect_lazy(&store.url)?;
				self
					.command
					.run::<_, PostgresEmployee, PostgresJob, PostgresLocation, PostgresOrganization, PostgresPerson, PostgresTimesheet>(
						pool,
						self.cascade_delete,
						config,
						self.delete,
						self.update,
					)
					.await
			},

			// NOTE: this is allowed because there may be additional adapters added later, and I want
			//       to define this behavior now.
			#[allow(unreachable_patterns)]
			_ => Err(AdapterError(store.adapter).into()),
		}
	}
}

mod command;

use clinvoice_adapter::{Adapters, Error as AdapterError, Store};
#[cfg(feature = "postgres")]
use clinvoice_adapter_postgres::data::{
	PostgresEmployee,
	PostgresJob,
	PostgresLocation,
	PostgresOrganization,
	PostgresPerson,
};
use clinvoice_config::Config;
use command::Command;
use structopt::StructOpt;

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
				self
					.command
					.run::<_, PostgresEmployee, PostgresJob, PostgresLocation, PostgresOrganization, PostgresPerson>(
						sqlx::PgPool::connect_lazy(&store.url)?,
						self.cascade_delete,
						config,
						self.delete,
						self.update,
					)
					.await
			},

			_ => return Err(AdapterError(store.adapter).into()),
		}
	}
}

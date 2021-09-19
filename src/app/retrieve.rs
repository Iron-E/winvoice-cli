mod command;

use command::Command;

use clinvoice_adapter::{Adapters, Error as AdapterError, Store};

use crate::DynResult;
use clinvoice_config::Config;
use structopt::StructOpt;

#[cfg(feature="postgres")]
use clinvoice_adapter_postgres::data::{PostgresEmployee, PostgresJob, PostgresLocation, PostgresOrganization, PostgresPerson};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(about = "Retrieve information that was recorded with CLInvoice")]
pub(super) struct Retrieve
{
	#[structopt(help = "Select retrieved entities for deletion. See -c", long, short)]
	pub delete: bool,

	#[structopt(
		help = "Cascade -d operations. Without this flag, entities referenced by other entities \
		        cannot be deleted",
		long = "cascade",
		short = "c",
	)]
	pub cascade_delete: bool,

	#[structopt(help = "Select retrieved entities for data updating", long, short)]
	pub update: bool,

	#[structopt(subcommand)]
	pub command: Command,
}

impl Retrieve
{
	/// # Summary
	///
	/// Execute the constructed command.
	pub(super) async fn run<'err>(
		self,
		config: &Config<'_, '_>,
		store: &Store,
	) -> DynResult<'err, ()>
	{
		match store.adapter
		{
			#[cfg(feature="postgres")]
			Adapters::Postgres => {
				self.command.run::<_, PostgresEmployee, PostgresJob, PostgresLocation, PostgresOrganization, PostgresPerson>(
					sqlx::PgPool::connect_lazy(&store.url)?,
					self.cascade_delete,
					config,
					self.delete,
					self.update,
				).await
			},

			_ => return Err(AdapterError(store.adapter).into()),
		}
	}
}

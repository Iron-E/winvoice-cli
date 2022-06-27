mod command;

use clinvoice_adapter_postgres::schema::PgTimesheet;
use clinvoice_config::{Adapters, Config, Error, Store};
use command::Command;
use structopt::StructOpt;
#[cfg(feature = "postgres")]
use {
	clinvoice_adapter_postgres::schema::{
		PgContactInfo,
		PgEmployee,
		PgJob,
		PgLocation,
		PgOrganization,
	},
	sqlx::PgPool,
};

use crate::DynResult;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(about = "Retrieve information that was recorded with CLInvoice")]
pub struct Retrieve
{
	#[structopt(help = "Select retrieved entities for deletion.", long, short)]
	delete: bool,

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
	pub async fn run(self, config: &Config, store: &Store) -> DynResult<()>
	{
		match store.adapter
		{
			#[cfg(feature = "postgres")]
			Adapters::Postgres =>
			{
				let pool = PgPool::connect_lazy(&store.url)?;
				self
					.command
					.run::<_, PgContactInfo, PgEmployee, PgJob, PgLocation, PgOrganization, PgTimesheet>(
						pool,
						config,
						self.delete,
						self.update,
					)
					.await
			},

			// NOTE: this is allowed because there may be additional adapters added later, and I want
			//       to define this behavior now.
			#[allow(unreachable_patterns)]
			_ => Err(Error::FeatureNotFound(store.adapter).into()),
		}
	}
}

mod command;

use clinvoice_config::{Adapters, Error, Store};
use clinvoice_schema::Id;
use command::Command;
use structopt::StructOpt;
#[cfg(feature = "postgres")]
use {
	clinvoice_adapter_postgres::schema::{PgEmployee, PgExpenses, PgJob, PgTimesheet},
	sqlx::PgPool,
};

use crate::DynResult;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(about = "Time information that was recorded with CLInvoice")]
pub struct Time
{
	#[structopt(subcommand)]
	pub command: Command,

	#[structopt(
		help = "Do work as the default `Employee`, as specified in your configuration",
		long = "default",
		short = "d"
	)]
	pub use_default_employee_id: bool,
}

impl Time
{
	/// # Summary
	///
	/// Execute the constructed command.
	pub async fn run(self, default_employee_id: Option<Id>, store: &Store) -> DynResult<()>
	{
		match store.adapter
		{
			#[cfg(feature = "postgres")]
			Adapters::Postgres =>
			{
				let pool = PgPool::connect_lazy(&store.url)?;
				self
					.command
					.run::<_, PgEmployee, PgJob, PgTimesheet, PgExpenses>(
						pool,
						if self.use_default_employee_id
						{
							default_employee_id
						}
						else
						{
							None
						},
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

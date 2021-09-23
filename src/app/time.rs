mod command;

use core::time::Duration;

use clinvoice_adapter::{Adapters, Error as AdapterError, Store};
#[cfg(feature = "postgres")]
use clinvoice_adapter_postgres::data::{PostgresEmployee, PostgresJob};
use clinvoice_data::{finance::Currency, Id};
use command::Command;
use structopt::StructOpt;

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
	pub async fn run<'err>(
		self,
		default_currency: Currency,
		default_employee_id: Id,
		store: &Store,
	) -> DynResult<'err, ()>
	{
		let provided_employee_id = if self.use_default_employee_id
		{
			Some(default_employee_id)
		}
		else
		{
			None
		};
		match store.adapter
		{
			#[cfg(feature = "postgres")]
			Adapters::Postgres =>
			{
				self
					.command
					.run::<_, PostgresEmployee, PostgresJob>(
						sqlx::PgPool::connect_lazy(&store.url)?,
						default_currency,
						provided_employee_id,
					)
					.await
			},

			// NOTE: this is allowed because there may be additional adapters added later, and I want
			//       to define this behavior now.
			#[allow(unreachable_patterns)]
			_ => return Err(AdapterError(store.adapter).into()),
		}
	}
}

mod command;

use command::Command;

use clinvoice_adapter::{Adapters, Error as AdapterError, Store};
use clinvoice_data::{Id, finance::Currency};
use core::time::Duration;

use crate::DynResult;
use structopt::StructOpt;

#[cfg(feature="postgres")]
use clinvoice_adapter_postgres::data::{PostgresEmployee, PostgresJob};

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
		default_timesheet_interval: Duration,
		store: &Store,
	) -> DynResult<'err, ()>
	{
		let provided_employee_id = if self.use_default_employee_id { Some(default_employee_id) } else { None };
		match store.adapter
		{
			#[cfg(feature="postgres")]
			Adapters::Postgres => {
				self.command.run::<_, PostgresEmployee, PostgresJob>(
					sqlx::PgPool::connect_lazy(&store.url)?,
					default_currency,
					provided_employee_id,
					default_timesheet_interval,
				).await
			},

			_ => return Err(AdapterError(store.adapter).into()),
		}
	}
}

mod command;
mod create;
mod retrieve;
mod time;

use clinvoice_config::Config;
use command::Command;
use create::Create;
use retrieve::Retrieve;
use structopt::StructOpt;
use time::Time;

use crate::DynResult;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(
	name = "clinvoice",
	about = "CLInvoice is a tool to help with invoicing from the command line!"
)]
pub struct App
{
	#[structopt(
		default_value = "default",
		help = "A store from the configuration file which operations should be performed on",
		long,
		short
	)]
	store: String,

	#[structopt(subcommand)]
	command: Command,
}

impl App
{
	/// # Summary
	///
	/// Run the application and parse its provided arguments / flags.
	pub async fn run<'err>(self, config: Config<'_, '_>) -> DynResult<'err, ()>
	{
		self
			.command
			.run(
				&config,
				config
					.get_store(&self.store)
					.expect("Storage name not known"),
			)
			.await
	}
}

mod command;
mod config;
mod create;
mod delete;
mod init;
mod match_args;
mod retrieve;
mod update;

use clap::Parser as Clap;
use clinvoice_config::Config;
use command::Command;

use crate::DynResult;

/// CLInvoice is a tool to track and generate invoices from the command line. Pass --help for more.
///
/// It is capable of managing information about clients, employees, jobs, timesheets, and exporting
/// the information into the format of your choice.
#[derive(Clap, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Args
{
	/// A key from the `[stores]` section of the [configuration file](clinvoice_config::Config).
	#[clap(
		default_value = "default",
		help = "A key from the `[stores]` section of the configuration file.",
		long,
		short
	)]
	store: String,

	#[clap(subcommand)]
	command: Command,
}

impl Args
{
	pub async fn run(self) -> DynResult<()>
	{
		let config = Config::read()?;
		let store = config.get_store(&self.store).cloned().ok_or_else(|| {
			format!(
				r#"The store named "{}" was not found in your configuration file."#,
				self.store,
			)
		})?;

		match self.command
		{
			Self::Config => config::edit(&config).map_err(|e| e.into()),
			Self::Create(cmd) => todo!(),
			Self::Delete(args) => todo!(),
			Self::Init => init::run(&store).await,
			Self::Retrieve(args) => todo!(),
			Self::Update(args) => todo!(),
		}
	}
}

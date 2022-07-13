mod command;
mod config;
mod create;
mod delete;
mod init;
mod match_args;
mod retrieve;
mod store_args;
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
	#[clap(subcommand)]
	command: Command,
}

impl Args
{
	pub async fn run(self) -> DynResult<()>
	{
		let config = Config::read()?;

		match self.command
		{
			Command::Config => config::edit(&config).map_err(|e| e.into()),
			Command::Create(create) => create.run(&config).await,
			Command::Delete(delete) => delete.run(&config).await,
			Command::Init(init) => init.run(&config).await,
			Command::Retrieve(retrieve) => retrieve.run(config).await,
			Command::Update(update) => update.run(&config).await,
		}
	}
}

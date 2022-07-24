mod command;
mod create;
mod delete;
mod init;
mod match_args;
mod retrieve;
mod run_action;
mod store_args;
mod update;

use clap::Parser as Clap;
use clinvoice_config::Config;
use command::Command;
use dialoguer::Editor;
use run_action::RunAction;

use crate::DynResult;

/// CLInvoice is a tool to track and generate invoices from the command line. Pass --help for more.
///
/// It is capable of managing information about clients, employees, jobs, timesheets, and exporting
/// the information into the format of your choice.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Args
{
	/// The specific CLInvoice subcommand to run.
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
			Command::Config =>
			{
				let serialized = toml::to_string_pretty(&config)?;
				if let Some(edited) = Editor::new().extension(".toml").edit(&serialized)?
				{
					let deserialized: Config = toml::from_str(&edited)?;
					deserialized.write()?;
				}
			},
			Command::Create(create) => create.run(config).await?,
			Command::Delete(delete) => delete.run(config).await?,
			Command::Init(init) => init.run(&config).await?,
			Command::Retrieve(retrieve) => retrieve.run(config).await?,
			Command::Update(update) => update.run(config).await?,
		};

		Ok(())
	}
}

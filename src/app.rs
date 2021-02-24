pub mod create;
pub mod retrieve;

use
{
	create::Create,
	retrieve::Retrieve,
	crate::{Config, io, StructOpt},
	clinvoice_adapter::{DynamicResult, data::Updatable},
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(name="clinvoice", about="CLInvoice is a tool to help with invoicing from the command line!")]
pub struct App
{
	#[structopt(about="Select retrieved entities for deletion", default_value="default", long, short)]
	store: String,

	#[structopt(subcommand)]
	command: AppCommand,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
enum AppCommand
{
	Config,

	Create(Create),

	Retrieve(Retrieve),
}

impl App
{
	/// # Summary
	///
	/// Edit the user's configuration file.
	fn edit_config(config: Config<'_, '_, '_, '_, '_, '_>) -> DynamicResult<()>
	{
		if let Some(edited) = io::input::toml_editor().edit(&toml::to_string_pretty(&config)?)?
		{
			toml::from_str::<Config>(&edited)?.update()?;
		};

		return Ok(());
	}

	/// # Summary
	///
	/// Run the application and parse its provided arguments / flags.
	pub fn run(self, config: Config<'_, '_, '_, '_, '_, '_>) -> DynamicResult<()>
	{
		return Ok(match self.command
		{
			AppCommand::Config => Self::edit_config(config),
			AppCommand::Create(cmd) => cmd.run(config, &self.store),
			AppCommand::Retrieve(cmd) => cmd.run(config, &self.store),
		}?);
	}
}

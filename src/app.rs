pub mod create;
pub mod retrieve;

use
{
	create::Create,
	retrieve::Retrieve,

	crate::{Config, DynResult, StructOpt},

	clinvoice_adapter::data::Updatable,
	clinvoice_config::Result as ConfigResult,

	dialoguer::Editor,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(name="clinvoice", about="CLInvoice is a tool to help with invoicing from the command line!")]
pub struct App
{
	#[structopt(default_value="default", help="A store from the configuration file which operations should be performed on", long, short)]
	store: String,

	#[structopt(subcommand)]
	command: AppCommand,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
enum AppCommand
{
	#[structopt(about="Edit the configuration file in the default editor")]
	Config,

	Create(Create),

	Retrieve(Retrieve),
}

impl App
{
	/// # Summary
	///
	/// Edit the user's configuration file.
	fn edit_config(config: &Config) -> ConfigResult<()>
	{
		if let Some(edited) = Editor::new().extension(".toml").edit(&toml::to_string_pretty(config)?)?
		{
			toml::from_str::<Config>(&edited)?.update()?;
		};

		Ok(())
	}

	/// # Summary
	///
	/// Run the application and parse its provided arguments / flags.
	pub fn run<'config>(self, config: &'config Config) -> DynResult<'config, ()>
	{
		match self.command
		{
			AppCommand::Config => Self::edit_config(config).map_err(|e| e.into()),
			AppCommand::Create(cmd) => cmd.run(config, self.store),
			AppCommand::Retrieve(cmd) => cmd.run(config, self.store),
		}
	}
}

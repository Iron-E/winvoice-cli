pub mod create;
pub mod retrieve;
pub mod time;

use clinvoice_adapter::data::Updatable;
use clinvoice_config::Result as ConfigResult;
use create::Create;
use dialoguer::Editor;
use retrieve::Retrieve;
use time::Time;

use crate::{Config, DynResult, StructOpt};

/// # Summary
///
/// The prompt for when editing a [query](clinvoice_query).
pub const QUERY_PROMPT: &str =
	"See the documentation of this query at https://github.com/Iron-E/clinvoice/wiki/Query-Syntax#";

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
	command: AppCommand,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
enum AppCommand
{
	#[structopt(about = "Edit the configuration file in the default editor")]
	Config,

	Create(Create),

	Retrieve(Retrieve),

	Time(Time),
}

impl App
{
	/// # Summary
	///
	/// Edit the user's configuration file.
	fn edit_config(config: &Config) -> ConfigResult<()>
	{
		let serialized = toml::to_string_pretty(config)?;
		if let Some(edited) = Editor::new().extension(".toml").edit(&serialized)?
		{
			let deserialized: Config = toml::from_str(&edited)?;
			deserialized.update()?;
		}

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
			AppCommand::Time(cmd) => cmd.run(config, self.store),
		}
	}
}

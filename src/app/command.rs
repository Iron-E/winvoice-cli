use clinvoice_config::{Config, Result as ConfigResult};
use dialoguer::Editor;
use structopt::StructOpt;

use super::{init, Create, Retrieve, Time};
use crate::DynResult;

#[derive(Clone, Debug, Eq, Hash, PartialEq, StructOpt)]
pub enum Command
{
	#[structopt(about = "Edit the CLInvoice configuration in your default editor")]
	Config,

	Create(Create),

	#[structopt(
		about = "Prepare the specified store (-s) for use with CLInvoice.\nWill not clobber \
		         existing data. Should only be run by administrators."
	)]
	Init,

	Retrieve(Retrieve),

	Time(Time),
}

impl Command
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
	pub async fn run(self, config: &Config, store_name: &str) -> DynResult<()>
	{
		let store = config
			.get_store(store_name)
			.ok_or_else(|| format!(r#""{store_name}" is not a valid store name."#))?;

		match self
		{
			Self::Config => Self::edit_config(config).map_err(|e| e.into()),
			Self::Create(cmd) =>
			{
				cmd.run(
					config.invoices.default_currency,
					config.timesheets.default_increment,
					store,
				)
				.await
			},
			Self::Init => init::run(store).await,
			Self::Retrieve(cmd) => cmd.run(config, store).await,
			Self::Time(cmd) => cmd.run(config.employees.id, store).await,
		}
	}
}

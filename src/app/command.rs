use clinvoice_adapter::Store;
use clinvoice_config::{Config, Result as ConfigResult};
use dialoguer::Editor;
use futures::future;
use structopt::StructOpt;

use super::{Create, Retrieve, Time};
use crate::DynResult;

#[derive(Clone, Debug, Eq, Hash, PartialEq, StructOpt)]
pub enum Command
{
	#[structopt(about = "Edit the configuration file in the default editor")]
	Config,

	Create(Create),

	// TODO: create `Init` command to prevent having to run `create table`
	//       commands as a new user.
	Retrieve(Retrieve),

	Time(Time),
}

impl Command
{
	/// # Summary
	///
	/// Edit the user's configuration file.
	fn edit_config(config: &Config<'_, '_>) -> ConfigResult<()>
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
	pub async fn run<'err>(self, config: &Config<'_, '_>, store: &Store) -> DynResult<'err, ()>
	{
		match self
		{
			Self::Config =>
			{
				match Self::edit_config(&config)
				{
					Ok(_) => future::ok(()),
					Err(e) => future::err(e.into()),
				}
				.await
			},
			Self::Create(cmd) =>
			{
				cmd.run(
					config.invoices.default_currency,
					config.timesheets.default_increment,
					store,
				)
				.await
			},
			Self::Retrieve(cmd) => cmd.run(&config, store).await,
			Self::Time(cmd) =>
			{
				cmd.run(
					config.invoices.default_currency,
					config.employees.default_id,
					store,
				)
				.await
			},
		}
	}
}

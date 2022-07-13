use clinvoice_config::{Config, Result};
use dialoguer::Editor;

/// Allow the user to edit their `config`uration file in their default editor.
pub fn edit(config: &Config) -> Result<()>
{
	let serialized = toml::to_string_pretty(config)?;
	if let Some(edited) = Editor::new().extension(".toml").edit(&serialized)?
	{
		let deserialized: Config = toml::from_str(&edited)?;
		deserialized.write()?;
	}

	Ok(())
}

use std::fs; // PERF: we're using `std::fs` because no more than 1 config will be written at a time

use clinvoice_adapter::data::Updatable;

use super::{Config, Error, Result};

#[async_trait::async_trait]
impl Updatable for Config<'_, '_>
{
	type Error = Error;

	async fn update(&self) -> Result<()>
	{
		let path = Self::path();

		// TODO: move this out into `Config::init`
		if let Some(parent) = path.parent()
		{
			if !parent.is_dir()
			{
				fs::create_dir_all(parent)?;
			}
		}

		let serialized = toml::to_string_pretty(self)?;
		fs::write(path, serialized)?;

		Ok(())
	}
}

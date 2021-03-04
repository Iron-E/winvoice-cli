use
{
	super::{Config, Error, Result},
	clinvoice_adapter::data::Updatable,
	std::fs,
};

impl Updatable for Config<'_, '_, '_>
{
	type Error = Error;

	fn update(&self) -> Result<()>
	{
		let path = Self::path();

		if let Some(parent) = path.parent()
		{
			if !parent.is_dir() { fs::create_dir_all(parent)?; }
		}

		fs::write(path, toml::to_string_pretty(self)?)?;

		Ok(())
	}
}

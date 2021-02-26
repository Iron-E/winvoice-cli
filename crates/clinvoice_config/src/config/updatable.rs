use
{
	super::Config,
	clinvoice_adapter::{data::Updatable, DynamicResult},
	std::fs,
};

impl Updatable for Config<'_, '_, '_, '_, '_, '_>
{
	fn update(&self) -> DynamicResult<()>
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

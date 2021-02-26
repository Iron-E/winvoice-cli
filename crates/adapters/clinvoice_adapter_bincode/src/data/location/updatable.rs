use
{
	super::BincodeLocation,
	clinvoice_adapter::{data::Updatable, DynamicResult},
	std::fs,
};

impl Updatable for BincodeLocation<'_, '_, '_>
{
	fn update(&self) -> DynamicResult<()>
	{
		fs::write(self.filepath(), bincode::serialize(&self.location)?)?;
		Ok(())
	}
}

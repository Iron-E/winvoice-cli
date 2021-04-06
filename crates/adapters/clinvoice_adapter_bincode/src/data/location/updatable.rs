use
{
	std::fs,

	super::BincodeLocation,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Updatable,
};

impl Updatable for BincodeLocation<'_, '_>
{
	type Error = Error;

	fn update(&self) -> Result<()>
	{
		fs::write(self.filepath(), bincode::serialize(&self.location)?)?;
		Ok(())
	}
}

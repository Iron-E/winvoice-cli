use
{
	std::fs,

	super::BincodeJob,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Updatable,
};

impl Updatable for BincodeJob<'_, '_>
{
	type Error = Error;

	fn update(&self) -> Result<()>
	{
		let serialized = bincode::serialize(&self.job)?;
		fs::write(self.filepath(), serialized)?;
		Ok(())
	}
}

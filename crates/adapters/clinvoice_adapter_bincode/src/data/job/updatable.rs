use
{
	super::BincodeJob,
	crate::data::{Error, Result},
	clinvoice_adapter::data::Updatable,
	std::fs,
};

impl Updatable for BincodeJob<'_, '_>
{
	type Error = Error;

	fn update(&self) -> Result<()>
	{
		fs::write(self.filepath(), bincode::serialize(&self.job)?)?;
		Ok(())
	}
}

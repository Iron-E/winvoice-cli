use
{
	super::BincodeJob,
	crate::data::Result,
	clinvoice_adapter::data::Updatable,
	std::fs,
};

impl Updatable for BincodeJob<'_, '_, '_>
{
	fn update(&self) -> Result<()>
	{
		fs::write(self.filepath(), bincode::serialize(&self.job)?)?;
		Ok(())
	}
}

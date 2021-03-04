use
{
	super::BincodeEmployee,
	crate::data::Result,
	clinvoice_adapter::data::Updatable,
	std::fs,
};

impl Updatable for BincodeEmployee<'_, '_, '_>
{
	fn update(&self) -> Result<()>
	{
		fs::write(self.filepath(), bincode::serialize(&self.employee)?)?;
		Ok(())
	}
}

use
{
	super::BincodeEmployee,
	crate::data::{Error, Result},
	clinvoice_adapter::data::Updatable,
	std::fs,
};

impl Updatable for BincodeEmployee<'_, '_>
{
	type Error = Error;

	fn update(&self) -> Result<()>
	{
		fs::write(self.filepath(), bincode::serialize(&self.employee)?)?;
		Ok(())
	}
}

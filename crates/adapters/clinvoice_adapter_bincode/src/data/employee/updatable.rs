use
{
	std::fs,

	super::BincodeEmployee,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Updatable,
};

impl Updatable for BincodeEmployee<'_, '_>
{
	type Error = Error;

	fn update(&self) -> Result<()>
	{
		let serialized = bincode::serialize(&self.employee)?;
		fs::write(self.filepath(), serialized)?;
		Ok(())
	}
}

use
{
	super::BincodeOrganization,
	crate::data::{Error, Result},
	clinvoice_adapter::data::Updatable,
	std::fs,
};

impl Updatable for BincodeOrganization<'_, '_, '_>
{
	type Error = Error;

	fn update(&self) -> Result<()>
	{
		fs::write(self.filepath(), bincode::serialize(&self.organization)?)?;
		Ok(())
	}
}

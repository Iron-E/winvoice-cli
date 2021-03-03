use
{
	super::BincodeOrganization,
	crate::data::{Error, Result},
	clinvoice_adapter::data::Updatable,
	std::fs,
};

impl Updatable<Error> for BincodeOrganization<'_, '_, '_>
{
	fn update(&self) -> Result<()>
	{
		fs::write(self.filepath(), bincode::serialize(&self.organization)?)?;
		Ok(())
	}
}

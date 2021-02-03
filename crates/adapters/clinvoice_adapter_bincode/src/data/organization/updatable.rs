use
{
	super::BincodeOrganization,
	clinvoice_adapter::data::Updatable,
	std::{error::Error, fs},
};

impl Updatable for BincodeOrganization<'_, '_, '_>
{
	fn update(&self) -> Result<(), Box<dyn Error>>
	{
		fs::write(self.filepath(), bincode::serialize(&self.organization)?)?;
		return Ok(());
	}
}

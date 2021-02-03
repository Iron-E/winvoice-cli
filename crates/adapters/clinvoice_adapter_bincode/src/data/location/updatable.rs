use
{
	super::BincodeLocation,
	clinvoice_adapter::data::Updatable,
	std::{error::Error, fs},
};

impl Updatable for BincodeLocation<'_, '_, '_>
{
	fn update(&self) -> Result<(), Box<dyn Error>>
	{
		fs::write(self.filepath(), bincode::serialize(&self.location)?)?;
		return Ok(());
	}
}

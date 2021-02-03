use
{
	super::BincodeJob,
	clinvoice_adapter::data::Updatable,
	std::{error::Error, fs},
};

impl Updatable for BincodeJob<'_, '_, '_>
{
	fn update(&self) -> Result<(), Box<dyn Error>>
	{
		fs::write(self.filepath(), bincode::serialize(&self.job)?)?;
		return Ok(());
	}
}

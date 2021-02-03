use
{
	super::BincodePerson,
	clinvoice_adapter::data::Updatable,
	std::{error::Error, fs},
};

impl Updatable for BincodePerson<'_, '_, '_>
{
	fn update(&self) -> Result<(), Box<dyn Error>>
	{
		fs::write(self.filepath(), bincode::serialize(&self.person)?)?;
		return Ok(());
	}
}

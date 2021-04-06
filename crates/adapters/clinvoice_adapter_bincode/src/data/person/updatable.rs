use
{
	std::fs,

	super::BincodePerson,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Updatable,
};

impl Updatable for BincodePerson<'_, '_>
{
	type Error = Error;

	fn update(&self) -> Result<()>
	{
		fs::write(self.filepath(), bincode::serialize(&self.person)?)?;
		Ok(())
	}
}

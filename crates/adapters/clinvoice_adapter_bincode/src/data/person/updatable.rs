use
{
	super::BincodePerson,
	crate::data::Result,
	clinvoice_adapter::data::Updatable,
	std::fs,
};

impl Updatable for BincodePerson<'_, '_, '_>
{
	fn update(&self) -> Result<()>
	{
		fs::write(self.filepath(), bincode::serialize(&self.person)?)?;
		Ok(())
	}
}

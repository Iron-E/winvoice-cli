use
{
	super::BincodePerson,
	clinvoice_adapter::{DynamicResult, data::Updatable},
	std::fs,
};

impl Updatable for BincodePerson<'_, '_, '_>
{
	fn update(&self) -> DynamicResult<()>
	{
		fs::write(self.filepath(), bincode::serialize(&self.person)?)?;
		return Ok(());
	}
}

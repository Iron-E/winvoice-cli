use
{
	super::BincodeEmployee,
	clinvoice_adapter::{data::Updatable, DynamicResult},
	std::fs,
};

impl Updatable for BincodeEmployee<'_, '_, '_>
{
	fn update(&self) -> DynamicResult<()>
	{
		fs::write(self.filepath(), bincode::serialize(&self.employee)?)?;
		return Ok(());
	}
}

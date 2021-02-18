use
{
	super::BincodeJob,
	clinvoice_adapter::{DynamicResult, data::Updatable},
	std::fs,
};

impl Updatable for BincodeJob<'_, '_, '_>
{
	fn update(&self) -> DynamicResult<()>
	{
		fs::write(self.filepath(), bincode::serialize(&self.job)?)?;
		return Ok(());
	}
}

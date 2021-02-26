use
{
	super::BincodeOrganization,
	clinvoice_adapter::{data::Updatable, DynamicResult},
	std::fs,
};

impl Updatable for BincodeOrganization<'_, '_, '_>
{
	fn update(&self) -> DynamicResult<()>
	{
		fs::write(self.filepath(), bincode::serialize(&self.organization)?)?;
		Ok(())
	}
}

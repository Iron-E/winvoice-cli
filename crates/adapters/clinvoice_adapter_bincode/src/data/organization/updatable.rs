use
{
	super::BincodeOrganization,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Updatable,

	tokio::fs,
};

#[async_trait::async_trait]
impl Updatable for BincodeOrganization<'_, '_>
{
	type Error = Error;

	async fn update(&self) -> Result<()>
	{
		let serialized = bincode::serialize(&self.organization)?;
		fs::write(self.filepath(), serialized).await?;
		Ok(())
	}
}

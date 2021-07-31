use
{
	super::BincodeLocation,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Updatable,

	tokio::fs,
};

#[async_trait::async_trait]
impl Updatable for BincodeLocation<'_, '_>
{
	type Error = Error;

	async fn update(&self) -> Result<()>
	{
		let serialized = bincode::serialize(&self.location)?;
		fs::write(self.filepath(), serialized).await?;
		Ok(())
	}
}

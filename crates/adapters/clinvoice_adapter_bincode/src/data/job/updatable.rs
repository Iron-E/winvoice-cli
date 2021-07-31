use
{
	super::BincodeJob,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Updatable,

	tokio::fs,
};

#[async_trait::async_trait]
impl Updatable for BincodeJob<'_, '_>
{
	type Error = Error;

	async fn update(&self) -> Result<()>
	{
		let serialized = bincode::serialize(&self.job)?;
		fs::write(self.filepath(), serialized).await?;
		Ok(())
	}
}

use
{
	super::BincodePerson,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Updatable,

	tokio::fs,
};

#[async_trait::async_trait]
impl Updatable for BincodePerson<'_, '_>
{
	type Error = Error;

	async fn update(&self) -> Result<()>
	{
		let serialized = bincode::serialize(&self.person)?;
		fs::write(self.filepath(), serialized).await?;
		Ok(())
	}
}

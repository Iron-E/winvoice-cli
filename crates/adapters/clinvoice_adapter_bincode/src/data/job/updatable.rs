use clinvoice_adapter::data::Updatable;
use tokio::fs;

use super::BincodeJob;
use crate::data::{Error, Result};

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

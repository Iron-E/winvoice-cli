use clinvoice_adapter::data::Updatable;
use tokio::fs;

use super::BincodeOrganization;
use crate::data::{Error, Result};

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

use clinvoice_adapter::data::Updatable;
use tokio::fs;

use super::BincodeEmployee;
use crate::data::{
	Error,
	Result,
};

#[async_trait::async_trait]
impl Updatable for BincodeEmployee<'_, '_>
{
	type Error = Error;

	async fn update(&self) -> Result<()>
	{
		let serialized = bincode::serialize(&self.employee)?;
		fs::write(self.filepath(), serialized).await?;
		Ok(())
	}
}

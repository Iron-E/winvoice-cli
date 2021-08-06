use clinvoice_adapter::data::Updatable;
use tokio::fs;

use super::BincodePerson;
use crate::data::{Error, Result};

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

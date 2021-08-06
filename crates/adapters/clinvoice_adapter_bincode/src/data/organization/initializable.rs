use clinvoice_adapter::{data::Initializable, Store};

use super::BincodeOrganization;
use crate::{
	data::{Error, Result},
	util,
};

#[async_trait::async_trait]
impl Initializable for BincodeOrganization<'_, '_>
{
	type Error = Error;

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	async fn init(store: &Store) -> Result<()>
	{
		util::create_store_dir(&Self::path(store)).await?;
		Ok(())
	}
}

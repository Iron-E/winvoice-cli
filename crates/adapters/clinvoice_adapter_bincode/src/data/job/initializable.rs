use
{
	super::BincodeJob,
	crate::
	{
		data::{Error, Result},
		util,
	},

	clinvoice_adapter::{data::Initializable, Store},
};

#[async_trait::async_trait]
impl Initializable for BincodeJob<'_, '_>
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

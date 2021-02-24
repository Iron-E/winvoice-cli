use
{
	super::BincodeOrganization,
	crate::util,
	clinvoice_adapter::{data::Initializable, DynamicResult, Store},
};

impl Initializable for BincodeOrganization<'_, '_, '_>
{
	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store) -> DynamicResult<()>
	{
		util::create_store_dir(&Self::path(store))?;
		return Ok(());
	}
}


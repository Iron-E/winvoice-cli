use
{
	super::BincodeLocation,
	crate::util,
	clinvoice_adapter::{data::Initializable, DynamicResult, Store},
};

impl<'pass, 'path, 'user> Initializable<'pass, 'path, 'user> for BincodeLocation<'pass, 'path, 'user>
{
	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store<'pass, 'path, 'user>) -> DynamicResult<()>
	{
		util::create_store_dir(&Self::path(store))?;
		return Ok(());
	}
}


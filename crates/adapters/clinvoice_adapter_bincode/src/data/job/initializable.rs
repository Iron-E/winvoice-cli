use
{
	super::BincodeJob,
	crate::util,
	clinvoice_adapter::{data::Initializable, Store},
	std::error::Error,
};

impl<'pass, 'path, 'user> Initializable<'pass, 'path, 'user> for BincodeJob<'pass, 'path, 'user>
{
	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store<'pass, 'path, 'user>) -> Result<(), Box<dyn Error>>
	{
		util::create_store_dir(&Self::path(store))?;
		return Ok(());
	}
}


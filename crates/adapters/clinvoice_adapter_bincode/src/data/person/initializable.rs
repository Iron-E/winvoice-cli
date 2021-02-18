use
{
	super::BincodePerson,
	crate::util,
	clinvoice_adapter::
	{
		DynamicResult,
		data::Initializable,
		Store,
	},
};

impl<'pass, 'path, 'user> Initializable<'pass, 'path, 'user> for BincodePerson<'pass, 'path, 'user>
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

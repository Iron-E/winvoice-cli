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

impl Initializable for BincodePerson<'_, '_, '_>
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

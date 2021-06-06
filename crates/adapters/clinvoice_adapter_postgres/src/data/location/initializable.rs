use
{
	super::PostgresLocation,
	crate::data::{Error, Result},

	clinvoice_adapter::{data::Initializable, Store},
};

impl Initializable for PostgresLocation<'_, '_>
{
	type Error = Error;

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store) -> Result<()>
	{
		todo!()
	}
}


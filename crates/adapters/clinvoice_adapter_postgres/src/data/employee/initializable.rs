use
{
	clinvoice_adapter::{data::Initializable, Store},

	super::PostgresEmployee,
	crate::data::{Error, Result},
};

impl Initializable for PostgresEmployee<'_, '_>
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


use
{
	super::PostgresJob,
	crate::data::{Error, Result},

	clinvoice_adapter::{data::Initializable, Store},
};

impl Initializable for PostgresJob<'_, '_>
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

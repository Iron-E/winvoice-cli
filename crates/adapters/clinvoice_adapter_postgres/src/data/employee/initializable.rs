use
{
	clinvoice_adapter::{data::Initializable, Store},

	super::PostgresEmployee,
	crate::data::{Error, Result},
};

#[async_trait::async_trait]
impl Initializable for PostgresEmployee<'_, '_>
{
	type Error = Error;

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	async fn init(store: &Store) -> Result<()>
	{
		todo!()
	}
}


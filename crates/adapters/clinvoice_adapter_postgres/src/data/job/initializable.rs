use
{
	super::PostgresJob,
	crate::data::{Error, Result},

	clinvoice_adapter::{data::Initializable, Store},
};

#[async_trait::async_trait]
impl Initializable for PostgresJob<'_, '_>
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

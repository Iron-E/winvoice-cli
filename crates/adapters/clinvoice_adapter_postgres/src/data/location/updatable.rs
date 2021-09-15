use
{
	super::PostgresLocation,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Updatable,
};

#[async_trait::async_trait]
impl Updatable for PostgresLocation<'_>
{
	type Error = Error;

	async fn update(&self) -> Result<()>
	{
		todo!()
	}
}

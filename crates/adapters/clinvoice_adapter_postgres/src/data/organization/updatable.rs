use
{
	super::PostgresOrganization,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Updatable,
};

#[async_trait::async_trait]
impl Updatable for PostgresOrganization<'_>
{
	type Error = Error;

	async fn update(&self) -> Result<()>
	{
		todo!()
	}
}

use
{
	super::PostgresEmployee,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Updatable,
};

#[async_trait::async_trait]
impl Updatable for PostgresEmployee<'_, '_>
{
	type Error = Error;

	async fn update(&self) -> Result<()>
	{
		todo!()
	}
}

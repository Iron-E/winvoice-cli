use
{
	super::PostgresPerson,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Updatable,
};

#[async_trait::async_trait]
impl Updatable for PostgresPerson<'_, '_>
{
	type Error = Error;

	async fn update(&self) -> Result<()>
	{
		todo!()
	}
}

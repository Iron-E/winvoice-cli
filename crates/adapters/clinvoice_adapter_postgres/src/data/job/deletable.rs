use
{
	super::PostgresJob,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Deletable,
};

#[async_trait::async_trait]
impl Deletable for PostgresJob<'_, '_>
{
	type Error = Error;

	async fn delete(&self, _cascade: bool) -> Result<()>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	#[tokio::test]
	async fn delete()
	{
	}
}

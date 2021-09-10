use
{
	std::borrow::Cow::Borrowed,

	super::PostgresEmployee,
	crate::data::{PostgresJob, Error, Result},

	clinvoice_adapter::data::{Deletable, Error as DataError, JobAdapter, Updatable},
	clinvoice_query as query,
};

#[async_trait::async_trait]
impl Deletable for PostgresEmployee<'_, '_>
{
	type Error = Error;

	async fn delete(&self, cascade: bool) -> Result<()>
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

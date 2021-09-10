use
{
	std::borrow::Cow::Borrowed,

	super::PostgresOrganization,
	crate::data::{PostgresEmployee, PostgresJob, Error, Result},

	clinvoice_adapter::data::{Deletable, EmployeeAdapter, Error as DataError, JobAdapter},
	clinvoice_data::Employee,
	clinvoice_query as query,
};

#[async_trait::async_trait]
impl Deletable for PostgresOrganization<'_, '_>
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

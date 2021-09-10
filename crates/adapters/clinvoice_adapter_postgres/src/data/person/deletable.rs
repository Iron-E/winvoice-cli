use
{
	std::{borrow::Cow::Borrowed, fs, io::ErrorKind},

	super::PostgresPerson,
	crate::data::{PostgresEmployee, Error, Result},

	clinvoice_adapter::data::{Deletable, EmployeeAdapter, Error as DataError},
	clinvoice_query as query,
};

#[async_trait::async_trait]
impl Deletable for PostgresPerson<'_, '_>
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

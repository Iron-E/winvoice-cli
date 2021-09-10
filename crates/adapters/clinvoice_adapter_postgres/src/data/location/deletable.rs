use
{
	std::{borrow::Cow::Borrowed},

	super::PostgresLocation,
	crate::data::{PostgresOrganization, Error, Result},

	clinvoice_adapter::data::{Deletable, Error as DataError, LocationAdapter, OrganizationAdapter},
	clinvoice_data::Location,
	clinvoice_query as query,
};

#[async_trait::async_trait]
impl Deletable for PostgresLocation<'_, '_>
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

use clinvoice_data::views::OrganizationView;

use
{
	super::PostgresOrganization,
	crate::data::{Error, Result},

	clinvoice_adapter::data::OrganizationAdapter,
	clinvoice_data::{Location, Organization},
	clinvoice_query as query,
};

#[async_trait::async_trait]
impl OrganizationAdapter for PostgresOrganization<'_>
{
	type Error = Error;

	async fn create(
		location: Location,
		name: String,
		pool: Self::Pool,
	) -> Result<Organization>
	{
		todo!()
	}

	async fn retrieve(
		query: &query::Organization,
		pool: Self::Pool,
	) -> Result<Vec<Organization>>
	{
		todo!()
	}

	async fn retrieve_view(
		query: &query::Organization,
		pool: Self::Pool,
	) -> Result<Vec<OrganizationView>>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	#[tokio::test]
	async fn create()
	{
	}

	#[tokio::test]
	async fn retrieve()
	{
	}
}

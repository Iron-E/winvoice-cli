use clinvoice_data::views::LocationView;

use
{
	super::PostgresLocation,
	crate::data::{Error, Result},

	clinvoice_adapter::data::LocationAdapter,
	clinvoice_data::Location,
	clinvoice_query as query,
};

#[async_trait::async_trait]
impl LocationAdapter for PostgresLocation<'_>
{
	type Error = Error;

	async fn create(
		name: String,
		pool: Self::Pool,
	) -> Result<Location>
	{
		todo!()
	}

	async fn create_inner(&self, name: String)
		-> Result<Location>
	{
		todo!()
	}

	async fn retrieve(
		query: &query::Location,
		pool: Self::Pool,
	) -> Result<Vec<Location>>
	{
		todo!()
	}

	async fn retrieve_view(
		query: &query::Location,
		pool: Self::Pool,
	) -> Result<Vec<LocationView>>
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

use std::collections::HashMap;

use clinvoice_data::{Contact, EmployeeStatus, Organization, Person, views::EmployeeView};

use
{
	super::PostgresEmployee,
	crate::data::{Error, Result},

	clinvoice_adapter::data::EmployeeAdapter,

	clinvoice_data::Employee,
	clinvoice_query as query,
};

#[async_trait::async_trait]
impl EmployeeAdapter for PostgresEmployee<'_>
{
	type Error = Error;

	async fn create(
		contact_info: HashMap<String, Contact>,
		organization: Organization,
		person: Person,
		status: EmployeeStatus,
		title: String,
		pool: Self::Pool,
	) -> Result<Employee>
	{
		todo!()
	}

	async fn retrieve(
		query: &query::Employee,
		pool: Self::Pool,
	) -> Result<Vec<Employee>>
	{
		todo!()
	}

	async fn retrieve_view(
		query: &query::Employee,
		pool: Self::Pool,
	) -> Result<Vec<EmployeeView>>
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

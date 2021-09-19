use std::collections::HashMap;

use clinvoice_data::{Contact, EmployeeStatus, Organization, Person, views::EmployeeView};
use sqlx::{Executor, Postgres, Result};

use
{
	super::PostgresEmployee,

	clinvoice_adapter::data::EmployeeAdapter,

	clinvoice_data::Employee,
	clinvoice_query as query,
};

#[async_trait::async_trait]
impl EmployeeAdapter for PostgresEmployee
{
	async fn create(
		connection: impl 'async_trait + Executor<'_, Database = Postgres>,
		contact_info: HashMap<String, Contact>,
		organization: Organization,
		person: Person,
		status: EmployeeStatus,
		title: String,
	) -> Result<Employee>
	{
		todo!()
	}

	async fn retrieve(
		connection: impl 'async_trait + Executor<'_, Database = Postgres>,
		query: &query::Employee,
	) -> Result<Vec<Employee>>
	{
		todo!()
	}

	async fn retrieve_view(
		connection: impl 'async_trait + Executor<'_, Database = Postgres>,
		query: &query::Employee,
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
		// TODO: write test
	}

	#[tokio::test]
	async fn retrieve()
	{
		// TODO: write test
	}

	#[tokio::test]
	async fn retrieve_view()
	{
		// TODO: write test
	}
}

use std::collections::HashMap;

use clinvoice_adapter::data::EmployeeAdapter;
use clinvoice_data::{
	views::EmployeeView,
	Contact,
	Employee,
	EmployeeStatus,
	Organization,
	Person,
};
use clinvoice_query as query;
use futures::Stream;
use sqlx::{Executor, Postgres, Result};

use super::PostgresEmployee;

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

	fn retrieve<'a, S>(
		connection: impl Executor<'a, Database = Postgres>,
		query: &query::Employee,
	) -> S
	where
		S: Stream<Item = Result<Employee>>,
	{
		todo!()
	}

	fn retrieve_view<'a, S>(
		connection: impl Executor<'a, Database = Postgres>,
		query: &query::Employee,
	) -> S
	where
		S: Stream<Item = Result<EmployeeView>>,
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

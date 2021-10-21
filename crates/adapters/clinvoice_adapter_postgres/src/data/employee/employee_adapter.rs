use std::{collections::HashMap, fmt::Write};

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
use sqlx::{Acquire, Executor, Postgres, Result};

use super::PostgresEmployee;

#[async_trait::async_trait]
impl EmployeeAdapter for PostgresEmployee
{
	async fn create(
		connection: impl 'async_trait + Acquire<'_, Database = Postgres> + Send,
		contact_info: HashMap<String, Contact>,
		organization: &Organization,
		person: &Person,
		status: EmployeeStatus,
		title: String,
	) -> Result<Employee>
	{
		let mut transaction = connection.begin().await?;

		let row = sqlx::query!(
			"INSERT INTO employees
				(organization_id, person_id, status, title)
			VALUES
				($1, $2, $3, $4)
			RETURNING id;",
			organization.id,
			person.id,
			status.as_str() as _,
			title,
		)
		.fetch_one(&mut transaction)
		.await?;

		const INSERT_VALUES_APPROX_LEN: u8 = 39;
		let mut contact_info_values =
			String::with_capacity((INSERT_VALUES_APPROX_LEN as usize) * contact_info.len());
		contact_info.iter().for_each(|(label, contact)| {
			write!(contact_info_values, "({}, {}, ", row.id, label).unwrap();
			match contact
			{
				Contact::Address {
					location_id,
					export,
				} => write!(
					contact_info_values,
					"{}, {}, NULL, NULL",
					export, location_id
				),
				Contact::Email { email, export } =>
				{
					write!(contact_info_values, "{}, NULL, {}, NULL", export, email)
				},
				Contact::Phone { phone, export } =>
				{
					write!(contact_info_values, "{}, NULL, NULL, {}", export, phone)
				},
			}
			.unwrap();
			write!(contact_info_values, "),").unwrap();
		});
		contact_info_values.pop(); // get rid of the trailing `,` since SQL can't handle that :/

		sqlx::query(&format!(
			"INSERT INTO contact_information
				(employee_id, label, export, location_id, email, phone)
			VALUES {};",
			contact_info_values,
		))
		.execute(&mut transaction)
		.await?;

		transaction.commit().await?;

		Ok(Employee {
			contact_info,
			id: row.id,
			organization_id: organization.id,
			person_id: person.id,
			status,
			title,
		})
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
	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn create()
	{
		// TODO: write test
	}

	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn retrieve_view()
	{
		// TODO: write test
	}
}

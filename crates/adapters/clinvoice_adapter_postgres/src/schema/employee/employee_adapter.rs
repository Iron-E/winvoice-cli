use std::{collections::HashMap, fmt::Write};

use clinvoice_adapter::schema::EmployeeAdapter;
use clinvoice_match::MatchEmployee;
use clinvoice_schema::{
	views::EmployeeView,
	Contact,
	Employee,
	EmployeeStatus,
	Id,
	Organization,
	Person,
};
use sqlx::{PgPool, Result};

use super::PostgresEmployee;

#[async_trait::async_trait]
impl EmployeeAdapter for PostgresEmployee
{
	async fn create(
		connection: &PgPool,
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
			status.as_str(),
			title,
		)
		.fetch_one(&mut transaction)
		.await?;

		const INSERT_VALUES_APPROX_LEN: u8 = 39;
		let mut contact_info_values =
			String::with_capacity((INSERT_VALUES_APPROX_LEN as usize) * contact_info.len());

		(0..contact_info.len()).map(|i| i * 6).for_each(|i| {
			write!(
				contact_info_values,
				"(${}, ${}, ${}, ${}, ${}, ${}),",
				i + 1,
				i + 2,
				i + 3,
				i + 4,
				i + 5,
				i + 6,
			)
			.unwrap()
		});
		contact_info_values.pop(); // get rid of the trailing `,` since SQL can't handle that :/

		contact_info
			.iter()
			.fold(
				sqlx::query(&format!(
					"INSERT INTO contact_information
					(employee_id, label, export, address_id, email, phone)
				VALUES {};",
					contact_info_values,
				)),
				|mut query, (label, contact)| {
					query = query.bind(row.id).bind(label);

					match contact
					{
						Contact::Address {
							location_id,
							export,
						} => query
							.bind(export)
							.bind(location_id)
							.bind(None::<String>)
							.bind(None::<String>),
						Contact::Email { email, export } => query
							.bind(export)
							.bind(None::<Id>)
							.bind(email)
							.bind(None::<String>),
						Contact::Phone { phone, export } => query
							.bind(export)
							.bind(None::<Id>)
							.bind(None::<String>)
							.bind(phone),
					}
				},
			)
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
		connection: &PgPool,
		match_condition: &MatchEmployee,
	) -> Result<Vec<EmployeeView>>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	use std::collections::HashMap;

	use clinvoice_adapter::schema::{LocationAdapter, OrganizationAdapter, PersonAdapter};
	use clinvoice_schema::{Contact, EmployeeStatus};

	use super::{EmployeeAdapter, PostgresEmployee};
	use crate::schema::{util, PostgresLocation, PostgresOrganization, PostgresPerson};

	/// TODO: use fuzzing
	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn create()
	{
		let connection = util::connect().await;

		let earth = PostgresLocation::create(&connection, "Earth".into())
			.await
			.unwrap();

		let organization =
			PostgresOrganization::create(&connection, &earth, "Some Organization".into())
				.await
				.unwrap();

		let person = PostgresPerson::create(&connection, "My Name".into())
			.await
			.unwrap();

		let mut contact_info = HashMap::new();
		contact_info.insert("Office".into(), Contact::Address {
			location_id: earth.id,
			export: false,
		});
		contact_info.insert("Work Email".into(), Contact::Email {
			email: "foo@bar.io".into(),
			export: true,
		});
		contact_info.insert("Office Phone".into(), Contact::Phone {
			phone: "555 223 5039".into(),
			export: true,
		});

		let employee = PostgresEmployee::create(
			&connection,
			contact_info,
			&organization,
			&person,
			EmployeeStatus::Employed,
			"Janitor".into(),
		)
		.await
		.unwrap();

		let row = sqlx::query!("SELECT * FROM employees WHERE id = $1;", employee.id)
			.fetch_one(&connection)
			.await
			.unwrap();

		let contact_info_row = sqlx::query!(
			"SELECT * FROM contact_information WHERE employee_id = $1;",
			employee.id
		)
		.fetch_all(&connection)
		.await
		.unwrap()
		.into_iter()
		.fold(HashMap::new(), |mut contact, row| {
			contact.insert(
				row.label,
				row.address_id
					.map(|id| Contact::Address {
						location_id: id,
						export: row.export,
					})
					.unwrap_or_else(|| {
						row.email
							.map(|e| Contact::Email {
								email: e,
								export: row.export,
							})
							.unwrap_or_else(|| Contact::Phone {
								phone: row.phone.unwrap(),
								export: row.export,
							})
					}),
			);
			contact
		});

		// Assert ::create writes accurately to the DB
		assert_eq!(employee.id, row.id);
		assert_eq!(employee.contact_info, contact_info_row);
		assert_eq!(employee.organization_id, row.organization_id);
		assert_eq!(organization.id, row.organization_id);
		assert_eq!(employee.person_id, row.person_id);
		assert_eq!(person.id, row.person_id);
		assert_eq!(employee.status, row.status.parse().unwrap());
		assert_eq!(employee.title, row.title);
	}

	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn retrieve_view()
	{
		// TODO: write test
	}
}

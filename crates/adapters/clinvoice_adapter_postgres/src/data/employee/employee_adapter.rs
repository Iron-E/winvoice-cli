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
			status.as_str(),
			title,
		)
		.fetch_one(&mut transaction)
		.await?;

		const INSERT_VALUES_APPROX_LEN: u8 = 39;
		let mut contact_info_values =
			String::with_capacity((INSERT_VALUES_APPROX_LEN as usize) * contact_info.len());
		contact_info.iter().for_each(|(label, contact)| {
			// FIXME: labels with apostrophe (e.g. "Tony's House" will break this
			write!(contact_info_values, "({}, '{}', ", row.id, label).unwrap();
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
					write!(contact_info_values, "{}, NULL, '{}', NULL", export, email)
				},
				Contact::Phone { phone, export } =>
				{
					write!(contact_info_values, "{}, NULL, NULL, '{}'", export, phone)
				},
			}
			.unwrap();
			write!(contact_info_values, "),").unwrap();
		});
		contact_info_values.pop(); // get rid of the trailing `,` since SQL can't handle that :/

		sqlx::query(&format!(
			"INSERT INTO contact_information
				(employee_id, label, export, address_id, email, phone)
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
	use std::collections::HashMap;

	use clinvoice_adapter::data::{
		Initializable,
		LocationAdapter,
		OrganizationAdapter,
		PersonAdapter,
	};
	use clinvoice_data::{Contact, EmployeeStatus};

	use super::{EmployeeAdapter, PostgresEmployee};
	use crate::data::{
		util,
		PostgresLocation,
		PostgresOrganization,
		PostgresPerson,
		PostgresSchema,
	};

	/// TODO: use fuzzing
	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn create()
	{
		let mut connection = util::connect().await;

		PostgresSchema::init(&mut connection).await.unwrap();

		let earth = PostgresLocation::create(&mut connection, "Earth".into())
			.await
			.unwrap();

		let organization =
			PostgresOrganization::create(&mut connection, &earth, "Some Organization".into())
				.await
				.unwrap();

		let person = PostgresPerson::create(&mut connection, "My Name".into())
			.await
			.unwrap();

		let mut contact_info = HashMap::new();
		contact_info.insert("Office".into(), Contact::Address {
			location_id: earth.id,
			export:      false,
		});
		contact_info.insert("Work Email".into(), Contact::Email {
			email:  "foo@bar.io".into(),
			export: true,
		});
		contact_info.insert("Office Phone".into(), Contact::Phone {
			phone:  "555 223 5039".into(),
			export: true,
		});

		let employee = PostgresEmployee::create(
			&mut connection,
			contact_info,
			&organization,
			&person,
			EmployeeStatus::Employed,
			"Janitor".into(),
		)
		.await
		.unwrap();

		let row = sqlx::query!("SELECT * FROM employees;")
		.fetch_one(&mut connection)
		.await
		.unwrap();

		let contact_info_row = sqlx::query!(
			"SELECT * FROM contact_information WHERE employee_id = $1;",
			employee.id
		)
		.fetch_all(&mut connection)
		.await
		.unwrap()
		.into_iter()
		.fold(HashMap::new(), |mut contact, row| {
			contact.insert(
				row.label,
				row.address_id
					.map(|id| Contact::Address {
						location_id: id,
						export:      row.export,
					})
					.unwrap_or_else(|| {
						row.email
							.map(|e| Contact::Email {
								email:  e,
								export: row.export,
							})
							.unwrap_or_else(|| Contact::Phone {
								phone:  row.phone.unwrap(),
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

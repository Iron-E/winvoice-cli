use core::fmt::Write;
use std::collections::HashMap;

use clinvoice_adapter::{schema::EmployeeAdapter, WriteWhereClause};
use clinvoice_match::MatchEmployee;
use clinvoice_schema::{Contact, Employee, Id, Organization, Person};
use futures::TryStreamExt;
use sqlx::{PgPool, Result};

use super::{columns::PgEmployeeColumns, PgEmployee};
use crate::{
	schema::{
		organization::columns::PgOrganizationColumns,
		person::columns::PgPersonColumns,
		PgLocation,
	},
	PgSchema as Schema,
};

#[async_trait::async_trait]
impl EmployeeAdapter for PgEmployee
{
	async fn create(
		connection: &PgPool,
		contact_info: HashMap<String, Contact>,
		organization: Organization,
		person: Person,
		status: String,
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
				VALUES {contact_info_values};",
				)),
				|mut query, (label, contact)| {
					query = query.bind(row.id).bind(label);

					match contact
					{
						Contact::Address { location, export } => query
							.bind(export)
							.bind(location.id)
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
			organization,
			person,
			status,
			title,
		})
	}

	async fn retrieve(connection: &PgPool, match_condition: MatchEmployee) -> Result<Vec<Employee>>
	{
		let id_match =
			PgLocation::retrieve_matching_ids(connection, &match_condition.organization.location);

		let mut query = String::from(
			"SELECT
				array_agg((C1.export, C1.label, C1.address_id, C1.email, C1.phone)) AS contact_info,
				E.id, E.organization_id, E.person_id, E.status, E.title,
				O.name AS organization_name, O.location_id,
				P.name
			FROM employees E
			JOIN contact_information C1 ON (C1.employee_id = E.id)
			JOIN organizations O ON (O.id = E.organization_id)
			JOIN people P ON (P.id = E.person_id)",
		);
		Schema::write_where_clause(
			Schema::write_where_clause(
				Schema::write_where_clause(
					Schema::write_where_clause(
						crate::schema::write_where_clause::write_contact_set_where_clause(
							connection,
							Default::default(),
							"C1",
							&match_condition.contact_info,
							&mut query,
						)
						.await?,
						"E",
						&match_condition,
						&mut query,
					),
					"O",
					&match_condition.organization,
					&mut query,
				),
				"P",
				&match_condition.person,
				&mut query,
			),
			"O.location_id",
			&id_match.await?,
			&mut query,
		);
		query.push_str(
			" GROUP BY
				C1.employee_id,
				E.id, E.organization_id, E.person_id, E.status, E.title,
				O.name, O.location_id,
				P.name;",
		);

		const COLUMNS: PgEmployeeColumns<'static> = PgEmployeeColumns {
			contact_info: "contact_info",
			id: "id",
			organization: PgOrganizationColumns {
				id: "organization_id",
				location_id: "location_id",
				name: "organization_name",
			},
			person: PgPersonColumns {
				name: "name",
				id: "person_id",
			},
			status: "status",
			title: "title",
		};

		sqlx::query(&query)
			.fetch(connection)
			.and_then(|row| async move { COLUMNS.row_to_view(connection, &row).await })
			.try_collect()
			.await
	}
}

#[cfg(test)]
mod tests
{
	use std::collections::HashMap;

	use clinvoice_adapter::schema::{LocationAdapter, OrganizationAdapter, PersonAdapter};
	use clinvoice_match::{MatchEmployee, MatchLocation, MatchOrganization, MatchPerson, MatchStr};
	use clinvoice_schema::Contact;
	use futures::TryStreamExt;

	use super::{EmployeeAdapter, PgEmployee};
	use crate::schema::{util, PgLocation, PgOrganization, PgPerson};

	/// TODO: use fuzzing
	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn create()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into())
			.await
			.unwrap();

		let organization =
			PgOrganization::create(&connection, earth.clone(), "Some Organization".into())
				.await
				.unwrap();

		let person = PgPerson::create(&connection, "My Name".into())
			.await
			.unwrap();

		let employee = PgEmployee::create(
			&connection,
			[
				("Office".into(), Contact::Address {
					location: earth,
					export: false,
				}),
				("Work Email".into(), Contact::Email {
					email: "foo@bar.io".into(),
					export: true,
				}),
				("Office's Email".into(), Contact::Phone {
					phone: "555 223 5039".into(),
					export: true,
				}),
			]
			.into_iter()
			.collect(),
			organization.clone(),
			person.clone(),
			"Employed".into(),
			"Janitor".into(),
		)
		.await
		.unwrap();

		let row = sqlx::query!("SELECT * FROM employees WHERE id = $1;", employee.id)
			.fetch_one(&connection)
			.await
			.unwrap();

		let contact_info_row = {
			let connection_borrow = &connection;
			sqlx::query!(
				"SELECT * FROM contact_information WHERE employee_id = $1;",
				employee.id
			)
			.fetch(&connection)
			.try_fold(HashMap::new(), |mut contact, row| async move {
				contact.insert(
					row.label,
					if let Some(id) = row.address_id
					{
						Contact::Address {
							location: PgLocation::retrieve_by_id(connection_borrow, id).await?,
							export: row.export,
						}
					}
					else if let Some(e) = row.email
					{
						Contact::Email {
							email: e,
							export: row.export,
						}
					}
					else
					{
						Contact::Phone {
							phone: row.phone.unwrap(),
							export: row.export,
						}
					},
				);
				Ok(contact)
			})
			.await
			.unwrap()
		};

		// Assert ::create writes accurately to the DB
		assert_eq!(employee.id, row.id);
		assert_eq!(employee.contact_info, contact_info_row);
		assert_eq!(employee.organization.id, row.organization_id);
		assert_eq!(organization.id, row.organization_id);
		assert_eq!(employee.person.id, row.person_id);
		assert_eq!(person.id, row.person_id);
		assert_eq!(employee.status, row.status);
		assert_eq!(employee.title, row.title);
	}

	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn retrieve()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into())
			.await
			.unwrap();

		let usa = PgLocation::create_inner(&connection, earth, "USA".into())
			.await
			.unwrap();

		let (arizona, utah) = futures::try_join!(
			PgLocation::create_inner(&connection, usa.clone(), "Arizona".into()),
			PgLocation::create_inner(&connection, usa.clone(), "Utah".into()),
		)
		.unwrap();

		let (organization, organization2) = futures::try_join!(
			PgOrganization::create(&connection, arizona.clone(), "Some Organization".into()),
			PgOrganization::create(&connection, utah.clone(), "Some Other Organizatión".into()),
		)
		.unwrap();

		let (person, person2) = futures::try_join!(
			PgPerson::create(&connection, "My Name".into()),
			PgPerson::create(&connection, "Another Gúy".into()),
		)
		.unwrap();

		let (employee, employee2) = futures::try_join!(
			PgEmployee::create(
				&connection,
				[
					("Remote Office".into(), Contact::Address {
						location: utah,
						export: false,
					}),
					("Work Email".into(), Contact::Email {
						email: "foo@bar.io".into(),
						export: true,
					}),
					("Office's Phone".into(), Contact::Phone {
						phone: "555 223 5039".into(),
						export: true,
					}),
				]
				.into_iter()
				.collect(),
				organization,
				person.clone(),
				"Employed".into(),
				"Janitor".into(),
			),
			PgEmployee::create(
				&connection,
				[
					("Favorite Pizza Place".into(), Contact::Address {
						location: arizona,
						export: false,
					}),
					("Work Email".into(), Contact::Email {
						email: "some_kind_of_email@f.com".into(),
						export: true,
					}),
					("Office's Phone".into(), Contact::Phone {
						phone: "555-555-8008".into(),
						export: true,
					}),
				]
				.into_iter()
				.collect(),
				organization2,
				person2,
				"Management".into(),
				"Assistant to Regional Manager".into(),
			),
		)
		.unwrap();

		assert_eq!(
			PgEmployee::retrieve(&connection, MatchEmployee {
				organization: MatchOrganization {
					name: employee.organization.name.clone().into(),
					location: MatchLocation {
						name: MatchStr::Or(vec![
							employee.organization.location.name.clone().into(),
							MatchStr::Contains(employee2.organization.location.name.into())
						]),
						..Default::default()
					},
					..Default::default()
				},
				person: MatchPerson {
					id: person.id.into(),
					..Default::default()
				},
				..Default::default()
			})
			.await
			.unwrap()
			.as_slice(),
			&[employee],
		);
	}
}

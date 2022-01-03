use std::collections::HashMap;
use core::fmt::Write;

use clinvoice_adapter::{schema::EmployeeAdapter, WriteWhereClause};
use clinvoice_match::MatchEmployee;
use clinvoice_schema::{
	views::{ContactView, EmployeeView, OrganizationView, PersonView},
	Contact,
	Employee,
	Id,
	Organization,
	Person,
};
use futures::TryStreamExt;
use sqlx::{Error, PgPool, Result, Row};

use super::PgEmployee;
use crate::{schema::PgLocation, PgSchema as Schema};

#[async_trait::async_trait]
impl EmployeeAdapter for PgEmployee
{
	async fn create(
		connection: &PgPool,
		contact_info: HashMap<String, Contact>,
		organization: &Organization,
		person: &Person,
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
		let id_match = PgLocation::retrieve_matching_ids(
			connection,
			&match_condition.organization.location,
		);
		let mut query = String::from(
			"SELECT
				array_agg((C.export, C.label, C.address_id, C.email, C.phone)) AS contact_info,
				E.id, E.organization_id, E.person_id, E.status, E.title,
				O.name AS organization_name, O.location_id,
				P.name
			FROM employees E
			JOIN contact_information C ON (C.employee_id = E.id)
			JOIN organizations O ON (O.id = E.organization_id)
			JOIN people P ON (P.id = E.person_id)",
		);
		Schema::write_where_clause(
			Schema::write_where_clause(Default::default(), "E", match_condition, &mut query),
			"O.location_id",
			&id_match.await?,
			&mut query,
		);
		query.push_str(
			" GROUP BY C.employee_id, E.id, E.organization_id, E.person_id, E.status, E.title, \
			 O.name, O.location_id, P.name;",
		);

		sqlx::query(&query)
			.fetch(connection)
			.and_then(|row| async move {
				Ok(EmployeeView {
					id: row.get("id"),
					organization: OrganizationView {
						id: row.get("organization_id"),
						name: row.get("organization_name"),
						location: PgLocation::retrieve_view_by_id(
							connection,
							row.get("location_id"),
						)
						.await?,
					},
					person: PersonView {
						id: row.get("person_id"),
						name: row.get("name"),
					},
					contact_info: {
						let vec: Vec<(_, _, _, _, _)> = row.get("contact_info");
						let mut map = HashMap::with_capacity(vec.len());
						for contact in vec
						{
							map.insert(
								contact.1,
								if let Some(id) = contact.2
								{
									ContactView::Address {
										location: PgLocation::retrieve_view_by_id(connection, id)
											.await?,
										export: contact.0,
									}
								}
								else if let Some(email) = contact.3
								{
									ContactView::Email {
										email,
										export: contact.0,
									}
								}
								else if let Some(phone) = contact.4
								{
									ContactView::Phone {
										export: contact.0,
										phone,
									}
								}
								else
								{
									return Err(Error::Decode(
										"Row of `contact_info` did not match any `Contact` equivalent".into(),
									));
								},
							);
						}
						map
					},
					status: row.get("status"),
					title: row.get("title"),
				})
			})
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
	use clinvoice_schema::{
		views::{ContactView, EmployeeView, LocationView, OrganizationView, PersonView},
		Contact,
	};

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
			PgOrganization::create(&connection, &earth, "Some Organization".into())
				.await
				.unwrap();

		let person = PgPerson::create(&connection, "My Name".into())
			.await
			.unwrap();

		let employee = PgEmployee::create(
			&connection,
			[
				("Office".into(), Contact::Address {
					location_id: earth.id,
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
			&organization,
			&person,
			"Employed".into(),
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
		assert_eq!(employee.status, row.status);
		assert_eq!(employee.title, row.title);
	}

	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn retrieve_view()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into())
			.await
			.unwrap();

		let usa = PgLocation::create_inner(&connection, &earth, "USA".into())
			.await
			.unwrap();

		let arizona = PgLocation::create_inner(&connection, &usa, "Arizona".into())
			.await
			.unwrap();

		let utah = PgLocation::create_inner(&connection, &usa, "Utah".into())
			.await
			.unwrap();

		let earth_view = LocationView {
			id: earth.id,
			name: earth.name.clone(),
			outer: None,
		};
		let usa_view = LocationView {
			id: usa.id,
			name: usa.name.clone(),
			outer: Some(earth_view.clone().into()),
		};
		let arizona_view = LocationView {
			id: arizona.id,
			name: arizona.name.clone(),
			outer: Some(usa_view.clone().into()),
		};
		let utah_view = LocationView {
			id: utah.id,
			name: utah.name.clone(),
			outer: Some(usa_view.clone().into()),
		};

		let organization =
			PgOrganization::create(&connection, &arizona, "Some Organization".into())
				.await
				.unwrap();
		let organization2 =
			PgOrganization::create(&connection, &utah, "Some Other Organizatión".into())
				.await
				.unwrap();

		let organization_view = OrganizationView {
			id: organization.id,
			name: organization.name.clone(),
			location: arizona_view.clone(),
		};
		let organization2_view = OrganizationView {
			id: organization2.id,
			name: organization2.name.clone(),
			location: utah_view.clone(),
		};

		let person = PgPerson::create(&connection, "My Name".into())
			.await
			.unwrap();
		let person2 = PgPerson::create(&connection, "Another Gúy".into())
			.await
			.unwrap();

		let person_view = PersonView {
			id: person.id,
			name: person.name.clone(),
		};
		let person2_view = PersonView {
			id: person2.id,
			name: person2.name.clone(),
		};

		let employee = PgEmployee::create(
			&connection,
			[
				("Remote Office".into(), Contact::Address {
					location_id: utah.id,
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
			&organization,
			&person,
			"Employed".into(),
			"Janitor".into(),
		)
		.await
		.unwrap();
		let employee2 = PgEmployee::create(
			&connection,
			[
				("Favorite Pizza Place".into(), Contact::Address {
					location_id: arizona.id,
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
			&organization2,
			&person2,
			"Management".into(),
			"Assistant to Regional Manager".into(),
		)
		.await
		.unwrap();

		let employee_view = EmployeeView {
			id: employee.id,
			contact_info: [
				("Remote Office".into(), ContactView::Address {
					location: utah_view,
					export: false,
				}),
				("Work Email".into(), ContactView::Email {
					email: "foo@bar.io".into(),
					export: true,
				}),
				("Office's Phone".into(), ContactView::Phone {
					phone: "555 223 5039".into(),
					export: true,
				}),
			]
			.into_iter()
			.collect(),
			organization: organization_view,
			person: person_view,
			status: "Employed".into(),
			title: "Janitor".into(),
		};
		let employee2_view = EmployeeView {
			id: employee2.id,
			contact_info: [
				("Favorite Pizza Place".into(), ContactView::Address {
					location: arizona_view,
					export: false,
				}),
				("Work Email".into(), ContactView::Email {
					email: "some_kind_of_email@f.com".into(),
					export: true,
				}),
				("Office's Phone".into(), ContactView::Phone {
					phone: "555-555-8008".into(),
					export: true,
				}),
			]
			.into_iter()
			.collect(),
			organization: organization2_view,
			person: person2_view,
			status: "Management".into(),
			title: "Assistant to Regional Manager".into(),
		};

		assert_eq!(
			PgEmployee::retrieve_view(&connection, &MatchEmployee {
				organization: MatchOrganization {
					name: employee_view.organization.name.clone().into(),
					location: MatchLocation {
						name: MatchStr::Or(vec![
							employee_view.organization.location.name.clone().into(),
							MatchStr::Contains(employee2_view.organization.location.name.into())
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
			&[employee_view],
		);
	}
}

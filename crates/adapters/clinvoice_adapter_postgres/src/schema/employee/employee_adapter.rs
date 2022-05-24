use clinvoice_adapter::{
	schema::{ContactInfoAdapter, EmployeeAdapter},
	WriteWhereClause,
};
use clinvoice_match::MatchEmployee;
use clinvoice_schema::{ContactKind, Employee, Id, Organization};
use futures::{TryFutureExt, TryStreamExt};
use sqlx::{PgPool, Result, Row};

use super::{columns::PgEmployeeColumns, PgEmployee};
use crate::{
	schema::{organization::columns::PgOrganizationColumns, PgContactInfo, PgLocation},
	PgSchema as Schema,
};

#[async_trait::async_trait]
impl EmployeeAdapter for PgEmployee
{
	async fn create(
		connection: &PgPool,
		contact_info: Vec<(bool, ContactKind, String)>,
		name: String,
		organization: Organization,
		status: String,
		title: String,
	) -> Result<Employee>
	{
		let mut transaction = connection.begin().await?;

		let row = sqlx::query!(
			"INSERT INTO employees
				(name, organization_id, status, title)
			VALUES
				($1, $2, $3, $4)
			RETURNING id;",
			name,
			organization.id,
			status,
			title,
		)
		.fetch_one(&mut transaction)
		.await?;

		let contact_info_db = PgContactInfo::create(&mut transaction, contact_info, row.id).await?;

		transaction.commit().await?;

		Ok(Employee {
			contact_info: contact_info_db,
			id: row.id,
			name,
			organization,
			status,
			title,
		})
	}

	async fn retrieve(connection: &PgPool, match_condition: MatchEmployee) -> Result<Vec<Employee>>
	{
		let contact_info_fut =
			PgContactInfo::retrieve(connection, match_condition.contact_info.clone());
		let id_match =
			PgLocation::retrieve_matching_ids(connection, &match_condition.organization.location);

		let mut query = String::from(
			"SELECT
				E.id, E.name, E.organization_id, E.status, E.title,
				O.name AS organization_name, O.location_id
			FROM employees E
			JOIN organizations O ON (O.id = E.organization_id)",
		);
		Schema::write_where_clause(
			Schema::write_where_clause(
				Schema::write_where_clause(Default::default(), "E", &match_condition, &mut query),
				"O",
				&match_condition.organization,
				&mut query,
			),
			"O.location_id",
			&id_match.await?,
			&mut query,
		);
		query.push(';');

		const COLUMNS: PgEmployeeColumns<'static> = PgEmployeeColumns {
			id: "id",
			organization: PgOrganizationColumns {
				id: "organization_id",
				location_id: "location_id",
				name: "organization_name",
			},
			name: "name",
			status: "status",
			title: "title",
		};

		let contact_info = &contact_info_fut.await?;
		sqlx::query(&query)
			.fetch(connection)
			.try_filter_map(|row| async move {
				match contact_info.get(&row.get::<Id, _>(COLUMNS.id))
				{
					Some(employee_contact_info) =>
					{
						COLUMNS
							.row_to_view(connection, employee_contact_info.clone(), &row)
							.map_ok(Some)
							.await
					},
					// If `PgContactInfo::retrieve` does not match, then the whole `employee` does not
					// match.
					_ => return Ok(None),
				}
			})
			.try_collect()
			.await
	}
}

#[cfg(test)]
mod tests
{
	use clinvoice_adapter::schema::{LocationAdapter, OrganizationAdapter};
	use clinvoice_match::{Match, MatchEmployee, MatchLocation, MatchOrganization, MatchSet};
	use clinvoice_schema::{Contact, ContactKind};
	use futures::TryStreamExt;

	use super::{EmployeeAdapter, PgEmployee};
	use crate::schema::{util, PgLocation, PgOrganization};

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

		let employee = PgEmployee::create(
			&connection,
			vec![
				(true, ContactKind::Address(earth), "Office".into()),
				(
					true,
					ContactKind::Email("foo@bar.io".into()),
					"Work Email".into(),
				),
				(
					true,
					ContactKind::Phone("555 223 5039".into()),
					"Office's Email".into(),
				),
			],
			"My Name".into(),
			organization.clone(),
			"Employed".into(),
			"Janitor".into(),
		)
		.await
		.unwrap();

		let row = sqlx::query!("SELECT * FROM employees WHERE id = $1;", employee.id)
			.fetch_one(&connection)
			.await
			.unwrap();

		let contact_info_row = async {
			let connection_borrow = &connection;
			sqlx::query!(
				"SELECT * FROM contact_information WHERE employee_id = $1;",
				employee.id
			)
			.fetch(connection_borrow)
			.and_then(|row| async move {
				Ok(Contact {
					employee_id: row.employee_id,
					export: row.export,
					label: row.label,
					kind: match row
						.email
						.map(ContactKind::Email)
						.or_else(|| row.phone.map(ContactKind::Phone))
					{
						Some(k) => k,
						_ => ContactKind::Address(
							PgLocation::retrieve_by_id(connection_borrow, row.address_id.unwrap()).await?,
						),
					},
				})
			})
			.try_collect::<Vec<_>>()
			.await
			.unwrap()
		};

		// Assert ::create writes accurately to the DB
		assert_eq!(employee.id, row.id);
		assert_eq!(employee.contact_info, contact_info_row.await);
		assert_eq!(employee.name, row.name);
		assert_eq!(employee.organization.id, row.organization_id);
		assert_eq!(organization.id, row.organization_id);
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

		let (employee, employee2) = futures::try_join!(
			PgEmployee::create(
				&connection,
				vec![
					(false, ContactKind::Address(utah), "Remote Office".into()),
					(
						true,
						ContactKind::Email("foo@bar.io".into()),
						"Work Email".into(),
					),
					(
						true,
						ContactKind::Phone("555 223 5039".into()),
						"Office's Phone".into(),
					),
				],
				"My Name".into(),
				organization,
				"Employed".into(),
				"Janitor".into(),
			),
			PgEmployee::create(
				&connection,
				Default::default(),
				"Another Gúy".into(),
				organization2,
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
						id: Match::Or(vec![
							employee.organization.location.id.into(),
							employee2.organization.location.id.into(),
						]),
						..Default::default()
					},
					..Default::default()
				},
				..Default::default()
			})
			.await
			.unwrap()
			.as_slice(),
			&[employee.clone()],
		);

		assert!(PgEmployee::retrieve(&connection, MatchEmployee {
			contact_info: MatchSet::Contains(Default::default()),
			organization: MatchOrganization {
				id: Match::Or(vec![
					employee.organization.id.into(),
					employee2.organization.id.into(),
				]),
				..Default::default()
			},
			..Default::default()
		})
		.await
		.unwrap()
		.into_iter()
		.all(|e| e.contact_info == employee.contact_info &&
			e.organization.name == employee.organization.name &&
			e.organization.location.name == employee.organization.location.name &&
			e.name == employee.name &&
			e.status == employee.status &&
			e.title == employee.title));

		assert!(PgEmployee::retrieve(&connection, MatchEmployee {
			contact_info: MatchSet::Not(MatchSet::Contains(Default::default()).into()),
			organization: MatchOrganization {
				id: Match::Or(vec![
					employee.organization.id.into(),
					employee2.organization.id.into(),
				]),
				..Default::default()
			},
			..Default::default()
		})
		.await
		.unwrap()
		.into_iter()
		.all(|e| e.contact_info == employee2.contact_info &&
			e.organization.name == employee2.organization.name &&
			e.organization.location.name == employee2.organization.location.name &&
			e.name == employee2.name &&
			e.status == employee2.status &&
			e.title == employee2.title));
	}
}

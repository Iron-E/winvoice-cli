use std::collections::HashMap;

use clinvoice_adapter::{
	schema::{EmployeeAdapter, OrganizationAdapter},
	WriteWhereClause,
};
use clinvoice_match::MatchEmployee;
use clinvoice_schema::{Employee, Id, Organization};
use futures::{future, TryFutureExt, TryStreamExt};
use sqlx::{PgPool, Result, Row};

use super::{columns::PgEmployeeColumns, PgEmployee};
use crate::{schema::PgOrganization, PgSchema as Schema};

#[async_trait::async_trait]
impl EmployeeAdapter for PgEmployee
{
	async fn create(
		connection: &PgPool,
		name: String,
		organization: Organization,
		status: String,
		title: String,
	) -> Result<Employee>
	{
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
		.fetch_one(connection)
		.await?;

		Ok(Employee {
			id: row.id,
			name,
			organization,
			status,
			title,
		})
	}

	async fn retrieve(connection: &PgPool, match_condition: MatchEmployee) -> Result<Vec<Employee>>
	{
		// TODO: separate into `retrieve_all() -> Vec` and `retrieve -> Stream` to skip `Vec`
		//       collection?
		let organizations_fut = PgOrganization::retrieve(connection, match_condition.organization)
			.map_ok(|vec| {
				vec.into_iter()
					.map(|o| (o.id, o))
					.collect::<HashMap<_, _>>()
			});

		let mut query =
			String::from("SELECT E.id, E.name, E.organization_id, E.status, E.title FROM employees E");
		Schema::write_where_clause(Default::default(), "E", &match_condition, &mut query);
		query.push(';');

		const COLUMNS: PgEmployeeColumns<'static> = PgEmployeeColumns {
			id: "id",
			name: "name",
			organization_id: "organization_id",
			status: "status",
			title: "title",
		};

		let organizations = organizations_fut.await?;
		sqlx::query(&query)
			.fetch(connection)
			.try_filter_map(|row| {
				if let Some(o) = organizations.get(&row.get::<Id, _>(COLUMNS.organization_id))
				{
					return match COLUMNS.row_to_view(o.clone(), &row)
					{
						Ok(e) => future::ok(Some(e)),
						Err(e) => future::err(e),
					};
				}

				future::ok(None)
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
	use clinvoice_schema::ContactKind;

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

		let organization = PgOrganization::create(
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
			earth.clone(),
			"Some Organization".into(),
		)
		.await
		.unwrap();

		let employee = PgEmployee::create(
			&connection,
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

		// Assert ::create writes accurately to the DB
		assert_eq!(employee.id, row.id);
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
			PgOrganization::create(
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
				arizona.clone(),
				"Some Organization".into(),
			),
			PgOrganization::create(
				&connection,
				Default::default(),
				utah.clone(),
				"Some Other Organizatión".into(),
			),
		)
		.unwrap();

		let (employee, employee2) = futures::try_join!(
			PgEmployee::create(
				&connection,
				"My Name".into(),
				organization,
				"Employed".into(),
				"Janitor".into(),
			),
			PgEmployee::create(
				&connection,
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

		assert_eq!(
			PgEmployee::retrieve(&connection, MatchEmployee {
				organization: MatchOrganization {
					contact_info: MatchSet::Contains(Default::default()),
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
			.as_slice(),
			&[employee.clone()]
		);

		assert_eq!(
			PgEmployee::retrieve(&connection, MatchEmployee {
				organization: MatchOrganization {
					contact_info: MatchSet::Not(MatchSet::Contains(Default::default()).into()),
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
			.as_slice(),
			&[employee2]
		);
	}
}

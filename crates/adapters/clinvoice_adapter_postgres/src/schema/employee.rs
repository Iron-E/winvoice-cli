use std::collections::HashMap;

use clinvoice_schema::views::{ContactView, EmployeeView};
use sqlx::{postgres::PgRow, Error, PgPool, Result, Row};

use super::{PgLocation, PgOrganization, PgPerson};

mod deletable;
mod employee_adapter;
mod updatable;

/// Implementor of the [`EmployeeAdapter`](clinvoice_adapter::schema::EmployeeAdapter) for the
/// Postgres database.
pub struct PgEmployee;

impl PgEmployee
{
	pub(super) async fn row_to_view(
		row: &PgRow,
		connection: &PgPool,
		contact_info: &str,
		id: &str,
		name: &str,
		organization_id: &str,
		organization_location_id: &str,
		organization_name: &str,
		person_id: &str,
		status: &str,
		title: &str,
	) -> Result<EmployeeView>
	{
		let organization = PgOrganization::row_to_view(
			row,
			connection,
			organization_id,
			organization_location_id,
			organization_name,
		);
		let mut futures = Vec::new();
		let vec: Vec<(_, _, _, _, _)> = row.get(contact_info);
		let mut map = HashMap::with_capacity(vec.len());
		vec.into_iter().try_for_each(
			|(export, label, contact_location_id, contact_email, contact_phone)| {
				let view = if let Some(contact_location_id) = contact_location_id
				{
					futures.push((
						export,
						label,
						PgLocation::retrieve_view_by_id(connection, contact_location_id),
					));
					return Ok(());
				}
				else if let Some(contact_email) = contact_email
				{
					ContactView::Email {
						email: contact_email,
						export,
					}
				}
				else if let Some(contact_phone) = contact_phone
				{
					ContactView::Phone {
						export,
						phone: contact_phone,
					}
				}
				else
				{
					return Err(Error::Decode(
						"Row of `contact_info` did not match any `Contact` equivalent".into(),
					));
				};

				map.insert(label, view);
				Ok(())
			},
		)?;

		Ok(EmployeeView {
			id: row.get(id),
			organization: organization.await?,
			person: PgPerson::row_to_view(row, person_id, name),
			contact_info: {
				for (export, label, future) in futures
				{
					map.insert(label, ContactView::Address {
						location: future.await?,
						export,
					});
				}

				map
			},
			status: row.get(status),
			title: row.get(title),
		})
	}
}

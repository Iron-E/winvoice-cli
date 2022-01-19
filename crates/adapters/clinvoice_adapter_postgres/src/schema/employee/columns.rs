use std::collections::HashMap;

use clinvoice_schema::views::{ContactView, EmployeeView};
use sqlx::{postgres::PgRow, Error, PgPool, Result, Row};

use crate::schema::{
	organization::columns::PgOrganizationColumns,
	person::columns::PgPersonColumns,
	PgLocation,
};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::schema) struct PgEmployeeColumns<'col>
{
	pub id: &'col str,
	pub organization: PgOrganizationColumns<'col>,
	pub person: PgPersonColumns<'col>,
}

impl PgEmployeeColumns<'_>
{
	pub(in crate::schema) async fn row_to_view(
		self,
		connection: &PgPool,
		row: &PgRow,
	) -> Result<EmployeeView>
	{
		let organization = self.organization.row_to_view(connection, row);

		let mut futures = Vec::new();
		let vec: Vec<(_, _, _, _, _)> = row.get("contact_info");
		let mut map = HashMap::with_capacity(vec.len());
		vec.into_iter().try_for_each(
			|(export, label, contact_location_id, contact_email, contact_phone)| {
				let view = if let Some(contact_location_id) = contact_location_id
				{
					return Ok(futures.push((
						export,
						label,
						PgLocation::retrieve_view_by_id(connection, contact_location_id),
					)));
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
			id: row.get(self.id),
			organization: organization.await?,
			person: self.person.row_to_view(row),
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
			status: row.get("status"),
			title: row.get("title"),
		})
	}
}

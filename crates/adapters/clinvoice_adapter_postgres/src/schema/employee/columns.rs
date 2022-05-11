use std::collections::HashMap;

use clinvoice_schema::{Contact, Employee};
use sqlx::{postgres::PgRow, Error, PgPool, Result, Row};

use crate::schema::{
	organization::columns::PgOrganizationColumns,
	person::columns::PgPersonColumns,
	PgLocation,
};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::schema) struct PgEmployeeColumns<'col>
{
	pub contact_info: &'col str,
	pub id: &'col str,
	pub organization: PgOrganizationColumns<'col>,
	pub person: PgPersonColumns<'col>,
	pub status: &'col str,
	pub title: &'col str,
}

impl PgEmployeeColumns<'_>
{
	pub(in crate::schema) async fn row_to_view(
		self,
		connection: &PgPool,
		row: &PgRow,
	) -> Result<Employee>
	{
		let organization = self.organization.row_to_view(connection, row);

		let mut address_futures = Vec::new();
		let mut map = HashMap::new();
		match row
			.try_get(self.contact_info)
			.and_then(|vec: Vec<(_, _, _, _, _)>| {
				map.reserve(vec.len());
				vec.into_iter().try_for_each(
					|(export, label, contact_location_id, contact_email, contact_phone)| {
						let view = if let Some(contact_location_id) = contact_location_id
						{
							return Ok(address_futures.push((
								export,
								label,
								PgLocation::retrieve_by_id(connection, contact_location_id),
							)));
						}
						else if let Some(contact_email) = contact_email
						{
							Contact::Email {
								email: contact_email,
								export,
							}
						}
						else if let Some(contact_phone) = contact_phone
						{
							Contact::Phone {
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
				)
			})
		{
			Ok(_) | Err(Error::ColumnNotFound(_)) => (),
			Err(e) => return Err(e),
		};

		Ok(Employee {
			id: row.get(self.id),
			organization: organization.await?,
			person: self.person.row_to_view(row),
			contact_info: {
				for (export, label, future) in address_futures
				{
					map.insert(label, Contact::Address {
						location: future.await?,
						export,
					});
				}

				map
			},
			status: row.get(self.status),
			title: row.get(self.title),
		})
	}
}

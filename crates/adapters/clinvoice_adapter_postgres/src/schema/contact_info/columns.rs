use clinvoice_schema::{Contact, ContactKind};
use sqlx::{postgres::PgRow, Error, PgPool, Result, Row};

use crate::schema::PgLocation;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::schema) struct PgContactColumns<'col>
{
	pub employee_id: &'col str,
	pub export: &'col str,
	pub label: &'col str,
	pub address_id: &'col str,
	pub email: &'col str,
	pub phone: &'col str,
}

impl PgContactColumns<'_>
{
	pub(in crate::schema) async fn row_to_view(
		self,
		connection: &PgPool,
		row: &PgRow,
	) -> Result<Contact>
	{
		Ok(Contact {
			label: row.get(self.label),
			export: row.get(self.export),
			employee_id: row.get(self.employee_id),
			kind: match row
				.get::<Option<_>, _>(self.email)
				.map(ContactKind::Email)
				.or_else(|| row.get::<Option<_>, _>(self.phone).map(ContactKind::Phone))
			{
				Some(kind) => kind,
				_ => ContactKind::Address(
					PgLocation::retrieve_by_id(
						connection,
						row.get::<Option<_>, _>(self.address_id).ok_or_else(|| {
							Error::Decode(
								"Row of `contact_info` did not match any `Contact` equivalent".into(),
							)
						})?,
					)
					.await?,
				),
			},
		})
	}
}

use clinvoice_schema::{Contact, ContactKind};
use futures::TryFutureExt;
use sqlx::{postgres::PgRow, Error, PgPool, Result, Row, error::UnexpectedNullError};

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
	) -> Result<Option<Contact>>
	{
		let label = match row.try_get(self.label)
		{
			Ok(l) => l,
			Err(Error::ColumnDecode { index: _, source: s }) if s.is::<UnexpectedNullError>() => return Ok(None),
			Err(e) => return Err(e),
		};
		let kind_fut = async {
			match row
				.get::<Option<_>, _>(self.email)
				.map(ContactKind::Email)
				.or_else(|| row.get::<Option<_>, _>(self.phone).map(ContactKind::Phone))
				.map(Ok)
			{
				Some(kind) => kind,
				_ =>
				{
					let address_id = row.get::<Option<_>, _>(self.address_id).ok_or_else(|| {
						Error::Decode(
							"Row of `contact_info` did not match any `Contact` equivalent".into(),
						)
					})?;
					PgLocation::retrieve_by_id(connection, address_id)
						.map_ok(|location| ContactKind::Address(location))
						.await
				},
			}
		};

		Ok(Some(Contact {
			label,
			export: row.get(self.export),
			employee_id: row.get(self.employee_id),
			kind: kind_fut.await?,
		}))
	}
}

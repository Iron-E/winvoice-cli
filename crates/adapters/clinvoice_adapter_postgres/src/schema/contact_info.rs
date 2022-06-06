use clinvoice_adapter::schema::columns::ContactColumns;
use clinvoice_schema::{Contact, ContactKind};
use futures::TryFutureExt;
use sqlx::{error::UnexpectedNullError, postgres::PgRow, Error, PgPool, Result, Row};

use super::PgLocation;

mod contact_info_adapter;
mod deletable;
mod updatable;

pub struct PgContactInfo;

impl PgContactInfo
{
	pub(in crate::schema) async fn row_to_view(
		connection: &PgPool,
		columns: ContactColumns<&str>,
		row: &PgRow,
	) -> Result<Option<Contact>>
	{
		let label = match row.try_get(columns.label)
		{
			Ok(l) => l,
			Err(Error::ColumnDecode {
				index: _,
				source: s,
			}) if s.is::<UnexpectedNullError>() => return Ok(None),
			Err(e) => return Err(e),
		};
		let kind_fut = async {
			match row
				.get::<Option<_>, _>(columns.email)
				.map(ContactKind::Email)
				.or_else(|| {
					row.get::<Option<_>, _>(columns.phone)
						.map(ContactKind::Phone)
				})
			{
				Some(kind) => Ok(kind),
				_ =>
				{
					let address_id = row.get::<Option<_>, _>(columns.address_id).ok_or_else(|| {
						Error::Decode(
							"Row of `contact_info` did not match any `Contact` equivalent".into(),
						)
					})?;
					PgLocation::retrieve_by_id(connection, address_id)
						.map_ok(ContactKind::Address)
						.await
				},
			}
		};

		Ok(Some(Contact {
			label,
			export: row.get(columns.export),
			organization_id: row.get(columns.organization_id),
			kind: kind_fut.await?,
		}))
	}
}

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
	pub(super) async fn row_to_view(
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

		Ok(Some(Contact {
			label,
			kind: match row.get::<Option<_>, _>(columns.address_id)
			{
				Some(id) =>
				{
					PgLocation::retrieve_by_id(connection, id)
						.map_ok(ContactKind::Address)
						.await?
				},
				_ => row
					.get::<Option<_>, _>(columns.email)
					.map(ContactKind::Email)
					.or_else(|| {
						row.get::<Option<_>, _>(columns.other)
							.map(ContactKind::Other)
					})
					.or_else(|| {
						row.get::<Option<_>, _>(columns.phone)
							.map(ContactKind::Phone)
					})
					.ok_or_else(|| {
						Error::Decode(
							"Row of `contact_info` did not match any `Contact` equivalent".into(),
						)
					})?,
			},
		}))
	}
}

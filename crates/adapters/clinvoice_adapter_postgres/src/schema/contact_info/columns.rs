use clinvoice_schema::{Contact, ContactKind};
use futures::TryFutureExt;
use sqlx::{error::UnexpectedNullError, postgres::PgRow, Error, PgPool, Result, Row};

use crate::schema::{typecast::PgTypeCast, PgLocation, PgScopedColumn};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::schema) struct PgContactColumns<T>
{
	pub address_id: T,
	pub email: T,
	pub export: T,
	pub label: T,
	pub organization_id: T,
	pub phone: T,
}

impl<T> PgContactColumns<T>
where
	T: Copy,
{
	/// # Summary
	///
	/// Returns an alternation of [`PgContactColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{ident}.{column}`.
	pub(in crate::schema) fn scoped<TIdent>(
		&self,
		ident: TIdent,
	) -> PgContactColumns<PgScopedColumn<T, TIdent>>
	where
		TIdent: Copy,
	{
		PgContactColumns {
			address_id: PgScopedColumn(ident, self.address_id),
			email: PgScopedColumn(ident, self.email),
			export: PgScopedColumn(ident, self.export),
			label: PgScopedColumn(ident, self.label),
			organization_id: PgScopedColumn(ident, self.organization_id),
			phone: PgScopedColumn(ident, self.phone),
		}
	}

	/// # Summary
	///
	/// Returns an alternation of [`PgContactColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{column}::{cast}`.
	pub(in crate::schema) fn typecast<TCast>(
		&self,
		cast: TCast,
	) -> PgContactColumns<PgTypeCast<TCast, T>>
	where
		TCast: Copy,
	{
		PgContactColumns {
			address_id: PgTypeCast(self.address_id, cast),
			email: PgTypeCast(self.email, cast),
			export: PgTypeCast(self.export, cast),
			label: PgTypeCast(self.label, cast),
			organization_id: PgTypeCast(self.organization_id, cast),
			phone: PgTypeCast(self.phone, cast),
		}
	}
}

impl PgContactColumns<&str>
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
			Err(Error::ColumnDecode {
				index: _,
				source: s,
			}) if s.is::<UnexpectedNullError>() => return Ok(None),
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
						.map_ok(ContactKind::Address)
						.await
				},
			}
		};

		Ok(Some(Contact {
			label,
			export: row.get(self.export),
			organization_id: row.get(self.organization_id),
			kind: kind_fut.await?,
		}))
	}
}

impl PgContactColumns<&'static str>
{
	pub(in crate::schema) const fn new() -> Self
	{
		Self {
			address_id: "address_id",
			email: "email",
			export: "export",
			label: "label",
			organization_id: "organization_id",
			phone: "phone",
		}
	}
}

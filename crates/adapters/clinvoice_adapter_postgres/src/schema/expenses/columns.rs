use clinvoice_schema::Expense;
use futures::TryFutureExt;
use sqlx::{error::UnexpectedNullError, postgres::PgRow, Error, PgPool, Result, Row};

use crate::schema::PgLocation;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::schema) struct PgExpenseColumns<'col>
{
	pub id: &'col str,
	pub timesheet_id: &'col str,
	pub category: &'col str,
	pub cost: &'col str,
	pub description: &'col str,
}

impl PgExpenseColumns<'_>
{
	pub(in crate::schema) async fn row_to_view(
		self,
		connection: &PgPool,
		row: &PgRow,
	) -> Result<Option<Expense>>
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
				.map(ExpenseKind::Email)
				.or_else(|| row.get::<Option<_>, _>(self.phone).map(ExpenseKind::Phone))
				.map(Ok)
			{
				Some(kind) => kind,
				_ =>
				{
					let address_id = row.get::<Option<_>, _>(self.address_id).ok_or_else(|| {
						Error::Decode(
							"Row of `contact_info` did not match any `Expense` equivalent".into(),
						)
					})?;
					PgLocation::retrieve_by_id(connection, address_id)
						.map_ok(|location| ExpenseKind::Address(location))
						.await
				},
			}
		};

		Ok(Some(Expense {
			label,
			export: row.get(self.export),
			employee_id: row.get(self.employee_id),
			kind: kind_fut.await?,
		}))
	}
}

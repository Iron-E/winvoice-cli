use clinvoice_finance::Money;
use clinvoice_schema::{Invoice, InvoiceDate, Job};
use sqlx::{postgres::PgRow, Error, Executor, Postgres, Result, Row};

use crate::schema::{organization::columns::PgOrganizationColumns, util};

pub(in crate::schema) struct PgJobColumns<'col>
{
	pub client: PgOrganizationColumns<'col>,
	pub id: &'col str,
}

impl PgJobColumns<'_>
{
	pub(in crate::schema) async fn row_to_view(
		self,
		connection: impl Executor<'_, Database = Postgres>,
		row: &PgRow,
	) -> Result<Job>
	{
		Ok(Job {
			id: row.get(self.id),
			client: self.client.row_to_view(connection, row).await?,
			date_close: row.get("date_close"),
			date_open: row.get("date_open"),
			increment: util::duration_from(row.get("increment"))?,
			invoice: Invoice {
				date: row
					.get::<Option<_>, _>("invoice_date_issued")
					.map(|d| InvoiceDate {
						issued: d,
						paid: row.get("invoice_date_paid"),
					}),
				hourly_rate: {
					let amount = row.get::<String, _>("invoice_hourly_rate");
					Money {
						amount: amount.parse().map_err(|e| {
							Error::Decode(
								format!(
									"Value `{amount}` of column `invoice_hourly_rate` is not validly \
									 formatted: {e}",
								)
								.into(),
							)
						})?,
						..Default::default()
					}
				},
			},
			notes: row.get("notes"),
			objectives: row.get("objectives"),
		})
	}
}

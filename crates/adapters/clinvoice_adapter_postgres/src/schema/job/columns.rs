use clinvoice_finance::Money;
use clinvoice_schema::{views::JobView, Invoice, InvoiceDate};
use sqlx::{postgres::PgRow, Error, Executor, Postgres, Result, Row};

use crate::schema::{organization::columns::PgOrganizationColumns, util};

pub(in crate::schema) struct PgJobColumns<'col>
{
	pub client: PgOrganizationColumns<'col>,
	pub date_close: &'col str,
	pub date_open: &'col str,
	pub id: &'col str,
	pub increment: &'col str,
	pub invoice_date_issued: &'col str,
	pub invoice_date_paid: &'col str,
	pub invoice_hourly_rate: &'col str,
	pub notes: &'col str,
	pub objectives: &'col str,
}

impl PgJobColumns<'_>
{
	pub(in crate::schema) async fn row_to_view(
		self,
		connection: impl Executor<'_, Database = Postgres>,
		row: &PgRow,
	) -> Result<JobView>
	{
		Ok(JobView {
			id: row.get(self.id),
			client: self.client.row_to_view(connection, row).await?,
			date_close: row.get(self.date_close),
			date_open: row.get(self.date_open),
			increment: util::duration_from(row.get(self.increment))?,
			invoice: Invoice {
				date: row
					.get::<Option<_>, _>(self.invoice_date_issued)
					.map(|d| InvoiceDate {
						issued: d,
						paid: row.get(self.invoice_date_paid),
					}),
				hourly_rate: {
					let amount = row.get::<String, _>(self.invoice_hourly_rate);
					Money {
						amount: amount.parse().map_err(|e| {
							Error::Decode(
								format!(
									"Value `{amount}` of column `{}` is not validly formatted: {e}",
									self.invoice_hourly_rate,
								)
								.into(),
							)
						})?,
						..Default::default()
					}
				},
			},
			notes: row.get(self.notes),
			objectives: row.get(self.objectives),
		})
	}
}

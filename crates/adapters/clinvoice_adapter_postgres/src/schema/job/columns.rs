use clinvoice_finance::{Decimal, Money};
use clinvoice_schema::{Invoice, InvoiceDate, Job, Organization};
use sqlx::{postgres::PgRow, Result, Row};

use crate::schema::util;

pub(in crate::schema) struct PgJobColumns<'col>
{
	pub client_id: &'col str,
	pub date_open: &'col str,
	pub date_close: &'col str,
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
	pub(in crate::schema) fn row_to_view(self, client: Organization, row: &PgRow) -> Result<Job>
	{
		Ok(Job {
			client,
			date_close: row.get(self.date_close),
			date_open: row.get(self.date_open),
			id: row.get(self.id),
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
						amount: amount
							.parse::<Decimal>()
							.map_err(|e| util::finance_err_to_sqlx(e.into()))?,
						..Default::default()
					}
				},
			},
			notes: row.get(self.notes),
			objectives: row.get(self.objectives),
		})
	}
}

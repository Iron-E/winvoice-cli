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
		let increment = util::duration_from(row.get(self.increment))?;
		let amount = row
			.get::<String, _>(self.invoice_hourly_rate)
			.parse::<Decimal>()
			.map_err(|e| util::finance_err_to_sqlx(e.into()))?;

		Ok(Job {
			client,
			date_close: row.get(self.date_close),
			date_open: row.get(self.date_open),
			id: row.get(self.id),
			increment,
			invoice: Invoice {
				date: row
					.get::<Option<_>, _>(self.invoice_date_issued)
					.map(|d| InvoiceDate {
						issued: d,
						paid: row.get(self.invoice_date_paid),
					}),
				hourly_rate: Money {
					amount,
					..Default::default()
				},
			},
			notes: row.get(self.notes),
			objectives: row.get(self.objectives),
		})
	}
}

impl PgJobColumns<'static>
{
	pub(in crate::schema) const fn new() -> Self
	{
		Self {
			client_id: "client_id",
			date_close: "date_close",
			date_open: "date_open",
			id: "id",
			increment: "increment",
			invoice_date_issued: "invoice_date_issued",
			invoice_date_paid: "invoice_date_paid",
			invoice_hourly_rate: "invoice_hourly_rate",
			notes: "notes",
			objectives: "objectives",
		}
	}
}

use clinvoice_adapter::schema::columns::JobColumns;
use clinvoice_finance::{Decimal, Money};
use clinvoice_schema::{Invoice, InvoiceDate, Job, Organization};
use sqlx::{postgres::PgRow, Result, Row};

use super::util;

mod deletable;
mod job_adapter;
mod updatable;

pub struct PgJob;

impl PgJob
{
	pub(in crate::schema) fn row_to_view(
		columns: JobColumns<&str>,
		row: &PgRow,
		client: Organization,
	) -> Result<Job>
	{
		let increment = util::duration_from(row.get(columns.increment))?;
		let amount = row
			.get::<String, _>(columns.invoice_hourly_rate)
			.parse::<Decimal>()
			.map_err(|e| util::finance_err_to_sqlx(e.into()))?;

		Ok(Job {
			client,
			date_close: row.get(columns.date_close),
			date_open: row.get(columns.date_open),
			id: row.get(columns.id),
			increment,
			invoice: Invoice {
				date: row
					.get::<Option<_>, _>(columns.invoice_date_issued)
					.map(|d| InvoiceDate {
						issued: d,
						paid: row.get(columns.invoice_date_paid),
					}),
				hourly_rate: Money {
					amount,
					..Default::default()
				},
			},
			notes: row.get(columns.notes),
			objectives: row.get(columns.objectives),
		})
	}
}

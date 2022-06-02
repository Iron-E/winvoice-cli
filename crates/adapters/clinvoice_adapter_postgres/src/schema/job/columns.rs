use core::fmt::Display;

use clinvoice_finance::{Decimal, Money};
use clinvoice_schema::{Invoice, InvoiceDate, Job, Organization};
use sqlx::{postgres::PgRow, Result, Row};

use crate::schema::{util, PgScopedColumn};

pub(in crate::schema) struct PgJobColumns<D>
where
	D: Display,
{
	pub client_id: D,
	pub date_open: D,
	pub date_close: D,
	pub id: D,
	pub increment: D,
	pub invoice_date_issued: D,
	pub invoice_date_paid: D,
	pub invoice_hourly_rate: D,
	pub notes: D,
	pub objectives: D,
}

impl<D> PgJobColumns<D>
where
	D: Copy + Display,
{
	/// # Summary
	///
	/// Returns an alternation of [`PgJobColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{alias}.{column}`.
	pub(in crate::schema) fn scoped<TIdent>(
		&self,
		ident: TIdent,
	) -> PgJobColumns<PgScopedColumn<D, TIdent>>
	where
		TIdent: Copy + Display,
	{
		PgJobColumns {
			client_id: PgScopedColumn(ident, self.client_id),
			date_open: PgScopedColumn(ident, self.date_open),
			date_close: PgScopedColumn(ident, self.date_close),
			id: PgScopedColumn(ident, self.id),
			increment: PgScopedColumn(ident, self.increment),
			invoice_date_issued: PgScopedColumn(ident, self.invoice_date_issued),
			invoice_date_paid: PgScopedColumn(ident, self.invoice_date_paid),
			invoice_hourly_rate: PgScopedColumn(ident, self.invoice_hourly_rate),
			notes: PgScopedColumn(ident, self.notes),
			objectives: PgScopedColumn(ident, self.objectives),
		}
	}
}

impl PgJobColumns<&str>
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

impl PgJobColumns<&'static str>
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

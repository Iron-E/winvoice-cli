use clinvoice_finance::Money;
use clinvoice_schema::{views::JobView, Invoice, InvoiceDate};
use sqlx::{postgres::PgRow, Error, Executor, Postgres, Result, Row};

use super::{util, PgOrganization};

mod deletable;
mod job_adapter;
mod updatable;

pub struct PgJob;

impl PgJob
{
	pub(super) async fn row_to_view(
		row: &PgRow,
		connection: impl Executor<'_, Database = Postgres>,
		date_close: &str,
		date_open: &str,
		id: &str,
		increment: &str,
		invoice_date_issued: &str,
		invoice_date_paid: &str,
		invoice_hourly_rate: &str,
		notes: &str,
		objectives: &str,
		organization_id: &str,
		organization_location_id: &str,
		organization_name: &str,
	) -> Result<JobView>
	{
		Ok(JobView {
			id: row.get(id),
			client: PgOrganization::row_to_view(
				row,
				connection,
				organization_id,
				organization_location_id,
				organization_name,
			)
			.await?,
			date_close: row.get(date_close),
			date_open: row.get(date_open),
			increment: util::duration_from(row.get(increment))?,
			invoice: Invoice {
				date: row
					.get::<Option<_>, _>(invoice_date_issued)
					.map(|d| InvoiceDate {
						issued: d,
						paid: row.get(invoice_date_paid),
					}),
				hourly_rate: {
					let amount = row.get::<String, _>(invoice_hourly_rate);
					Money {
						amount: amount.parse().map_err(|e| {
							Error::Decode(
								format!(
									"Value `{amount}` of column `{invoice_hourly_rate}` is not validly \
									 formatted: {e}",
								)
								.into(),
							)
						})?,
						..Default::default()
					}
				},
			},
			notes: row.get(notes),
			objectives: row.get(objectives),
		})
	}
}

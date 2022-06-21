use clinvoice_adapter::schema::columns::{JobColumns, OrganizationColumns};
use clinvoice_finance::{Decimal, Money};
use clinvoice_schema::{Invoice, InvoiceDate, Job};
use sqlx::{postgres::PgRow, Executor, Postgres, Result, Row};

use super::{util, PgOrganization};

mod deletable;
mod job_adapter;
mod updatable;

pub struct PgJob;

impl PgJob
{
	pub(super) async fn row_to_view<TJobColumns, TOrgColumns>(
		connection: impl Executor<'_, Database = Postgres>,
		columns: JobColumns<TJobColumns>,
		organization_columns: OrganizationColumns<TOrgColumns>,
		row: &PgRow,
	) -> Result<Job>
	where
		TJobColumns: AsRef<str>,
		TOrgColumns: AsRef<str>,
	{
		let client_fut = PgOrganization::row_to_view(connection, organization_columns, row);
		let increment = util::duration_from(row.get(columns.increment.as_ref()))?;
		let amount = row
			.get::<String, _>(columns.invoice_hourly_rate.as_ref())
			.parse::<Decimal>()
			.map_err(|e| util::finance_err_to_sqlx(e.into()))?;

		Ok(Job {
			client: client_fut.await?,
			date_close: row.get(columns.date_close.as_ref()),
			date_open: row.get(columns.date_open.as_ref()),
			id: row.get(columns.id.as_ref()),
			increment,
			invoice: Invoice {
				date: row
					.get::<Option<_>, _>(columns.invoice_date_issued.as_ref())
					.map(|d| InvoiceDate {
						issued: d,
						paid: row.get(columns.invoice_date_paid.as_ref()),
					}),
				hourly_rate: Money {
					amount,
					..Default::default()
				},
			},
			notes: row.get(columns.notes.as_ref()),
			objectives: row.get(columns.objectives.as_ref()),
		})
	}
}

mod columns_to_sql;

use crate::fmt::{TypeCast, WithIdentifier};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct JobColumns<T>
{
	pub client_id: T,
	pub date_close: T,
	pub date_open: T,
	pub id: T,
	pub increment: T,
	pub invoice_date_issued: T,
	pub invoice_date_paid: T,
	pub invoice_hourly_rate: T,
	pub notes: T,
	pub objectives: T,
}

impl<T> JobColumns<T>
{
	/// # Summary
	///
	/// Returns an alternation of [`JobColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{ident}.{column}`.
	pub fn scoped<TIdent>(self, ident: TIdent) -> JobColumns<WithIdentifier<T, TIdent>>
	where
		TIdent: Copy,
	{
		JobColumns {
			client_id: WithIdentifier(ident, self.client_id),
			date_open: WithIdentifier(ident, self.date_open),
			date_close: WithIdentifier(ident, self.date_close),
			id: WithIdentifier(ident, self.id),
			increment: WithIdentifier(ident, self.increment),
			invoice_date_issued: WithIdentifier(ident, self.invoice_date_issued),
			invoice_date_paid: WithIdentifier(ident, self.invoice_date_paid),
			invoice_hourly_rate: WithIdentifier(ident, self.invoice_hourly_rate),
			notes: WithIdentifier(ident, self.notes),
			objectives: WithIdentifier(ident, self.objectives),
		}
	}

	/// # Summary
	///
	/// Returns an alternation of [`JobColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{column}::{cast}`.
	pub fn typecast<TCast>(self, cast: TCast) -> JobColumns<TypeCast<TCast, T>>
	where
		TCast: Copy,
	{
		JobColumns {
			client_id: TypeCast(self.client_id, cast),
			date_open: TypeCast(self.date_open, cast),
			date_close: TypeCast(self.date_close, cast),
			id: TypeCast(self.id, cast),
			increment: TypeCast(self.increment, cast),
			invoice_date_issued: TypeCast(self.invoice_date_issued, cast),
			invoice_date_paid: TypeCast(self.invoice_date_paid, cast),
			invoice_hourly_rate: TypeCast(self.invoice_hourly_rate, cast),
			notes: TypeCast(self.notes, cast),
			objectives: TypeCast(self.objectives, cast),
		}
	}
}

impl JobColumns<&'static str>
{
	pub const fn default() -> Self
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

	pub const fn unique() -> Self
	{
		Self {
			client_id: "unique_4_job_client_id",
			date_close: "unique_4_job_date_close",
			date_open: "unique_4_job_date_open",
			id: "unique_4_job_id",
			increment: "unique_4_job_increment",
			invoice_date_issued: "unique_4_job_invoice_date_issued",
			invoice_date_paid: "unique_4_job_invoice_date_paid",
			invoice_hourly_rate: "unique_4_job_invoice_hourly_rate",
			notes: "unique_4_job_notes",
			objectives: "unique_4_job_objectives",
		}
	}
}

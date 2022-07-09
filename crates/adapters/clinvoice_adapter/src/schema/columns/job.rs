mod columns_to_sql;
mod table_to_sql;

use crate::fmt::{As, TableToSql, TypeCast, WithIdentifier};

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
	/// Returns a [`JobColumns`] which outputs all of its columns as
	/// `column_1 AS aliased_column_1`.
	pub fn r#as<TAlias>(self, aliased: JobColumns<TAlias>) -> JobColumns<As<T, TAlias>>
	{
		JobColumns {
			client_id: As(self.client_id, aliased.client_id),
			date_close: As(self.date_close, aliased.date_close),
			date_open: As(self.date_open, aliased.date_open),
			id: As(self.id, aliased.id),
			increment: As(self.increment, aliased.increment),
			invoice_date_issued: As(self.invoice_date_issued, aliased.invoice_date_issued),
			invoice_date_paid: As(self.invoice_date_paid, aliased.invoice_date_paid),
			invoice_hourly_rate: As(self.invoice_hourly_rate, aliased.invoice_hourly_rate),
			notes: As(self.notes, aliased.notes),
			objectives: As(self.objectives, aliased.objectives),
		}
	}

	/// # Summary
	///
	/// Add a [scope](Self::scope) using the [default alias](TableToSql::default_alias)
	pub fn default_scope(self) -> JobColumns<WithIdentifier<T, char>>
	{
		self.scope(Self::DEFAULT_ALIAS)
	}

	/// # Summary
	///
	/// Returns a [`JobColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{ident}.{column}`.
	pub fn scope<TAlias>(self, alias: TAlias) -> JobColumns<WithIdentifier<T, TAlias>>
	where
		TAlias: Copy,
	{
		JobColumns {
			client_id: WithIdentifier(alias, self.client_id),
			date_open: WithIdentifier(alias, self.date_open),
			date_close: WithIdentifier(alias, self.date_close),
			id: WithIdentifier(alias, self.id),
			increment: WithIdentifier(alias, self.increment),
			invoice_date_issued: WithIdentifier(alias, self.invoice_date_issued),
			invoice_date_paid: WithIdentifier(alias, self.invoice_date_paid),
			invoice_hourly_rate: WithIdentifier(alias, self.invoice_hourly_rate),
			notes: WithIdentifier(alias, self.notes),
			objectives: WithIdentifier(alias, self.objectives),
		}
	}

	/// # Summary
	///
	/// Returns a [`JobColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{column}::{cast}`.
	pub fn typecast<TCast>(self, cast: TCast) -> JobColumns<TypeCast<T, TCast>>
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

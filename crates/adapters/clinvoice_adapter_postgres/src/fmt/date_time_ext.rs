use clinvoice_schema::{
	chrono::{DateTime, SubsecRound, TimeZone},
	Invoice,
	InvoiceDate,
	Job,
	Timesheet,
};

/// An extension to the [`DateTime`] which provides utility methods that aid in ensuring
/// that a given value is compatible with Postgres.
pub trait DateTimeExt
{
	/// Ensure that all dates/times contained in this type are not more precise than the postgres
	/// database can store.
	fn pg_sanitize(self) -> Self;
}

impl<T> DateTimeExt for DateTime<T>
where
	T: TimeZone,
{
	fn pg_sanitize(self) -> Self
	{
		self.trunc_subsecs(6)
	}
}

impl DateTimeExt for Invoice
{
	fn pg_sanitize(self) -> Self
	{
		Self {
			date: self.date.pg_sanitize(),
			..self
		}
	}
}

impl DateTimeExt for InvoiceDate
{
	fn pg_sanitize(self) -> Self
	{
		Self {
			issued: self.issued.pg_sanitize(),
			paid: self.paid.pg_sanitize(),
		}
	}
}

impl DateTimeExt for Job
{
	fn pg_sanitize(self) -> Self
	{
		Self {
			date_close: self.date_close.pg_sanitize(),
			date_open: self.date_open.pg_sanitize(),
			invoice: self.invoice.pg_sanitize(),
			..self
		}
	}
}

impl<T> DateTimeExt for Option<T>
where
	T: DateTimeExt,
{
	fn pg_sanitize(self) -> Self
	{
		self.map(DateTimeExt::pg_sanitize)
	}
}

impl DateTimeExt for Timesheet
{
	fn pg_sanitize(self) -> Self
	{
		Self {
			time_begin: self.time_begin.pg_sanitize(),
			time_end: self.time_end.pg_sanitize(),
			..self
		}
	}
}

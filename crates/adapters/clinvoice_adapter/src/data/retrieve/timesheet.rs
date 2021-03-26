use
{
	super::Employee,
	crate::data::MatchWhen,
	clinvoice_data::chrono::{DateTime, Utc},
};

/// # Summary
///
/// An [`Timesheet`](clinvoice_data::Timesheet) with [matchable](MatchWhen) fields.
pub struct Timesheet<'m>
{
	pub begin: MatchWhen<'m, DateTime<Utc>>,
	pub employee: Employee<'m>,
	pub end: MatchWhen<'m, Option<DateTime<Utc>>>,
}

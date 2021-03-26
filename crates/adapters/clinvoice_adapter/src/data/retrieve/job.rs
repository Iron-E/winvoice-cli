use
{
	super::{Invoice, Organization, Timesheet},
	crate::data::MatchWhen,
	clinvoice_data::
	{
		chrono::{DateTime, Utc},
		Id,
	},
};

/// # Summary
///
/// An [`Job`](clinvoice_data::Job) with [matchable](MatchWhen) fields.
pub struct Job<'m>
{
	pub client: Organization<'m>,
	pub date_close: MatchWhen<'m, Option<DateTime<Utc>>>,
	pub date_open: MatchWhen<'m, DateTime<Utc>>,
	pub id: MatchWhen<'m, Id>,
	pub invoice: Invoice<'m>,
	pub notes: MatchWhen<'m, String>,
	pub objectives: MatchWhen<'m, String>,
	pub timesheet: Timesheet<'m>,
}

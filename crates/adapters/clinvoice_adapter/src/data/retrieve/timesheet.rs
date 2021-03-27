use
{
	super::Employee,
	crate::data::MatchWhen,
	clinvoice_data::
	{
		chrono::{DateTime, Utc},
		views::TimesheetView,
	},
};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// An [`Timesheet`](clinvoice_data::Timesheet) with [matchable](MatchWhen) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Timesheet<'m>
{
	#[cfg_attr(feature="serde_support", serde(default))]
	pub begin: MatchWhen<'m, DateTime<Utc>>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub employee: Employee<'m>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub end: MatchWhen<'m, Option<DateTime<Utc>>>,
}

impl Timesheet<'_>
{
	/// # Summary
	///
	/// Return `true` if `timesheet` is a match.
	pub fn matches(&self, timesheet: &clinvoice_data::Timesheet) -> bool
	{
		todo!()
	}

	/// # Summary
	///
	/// Return `true` if `timesheet` is a match.
	pub fn matches_view(&self, timeseet: &TimesheetView) -> bool
	{
		todo!()
	}
}

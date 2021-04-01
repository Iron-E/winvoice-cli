use
{
	super::Employee,
	crate::data::Match,
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
/// An [`Timesheet`](clinvoice_data::Timesheet) with [matchable](Match) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Timesheet<'m>
{
	#[cfg_attr(feature="serde_support", serde(default))]
	pub employee: Employee<'m>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub time_begin: Match<'m, DateTime<Utc>>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub time_end: Match<'m, Option<DateTime<Utc>>>,
}

impl Timesheet<'_>
{
	/// # Summary
	///
	/// Return `true` if `timesheet` is a match.
	pub fn any_matches_view(&self, timesheets: &[TimesheetView]) -> bool
	{
		timesheets.iter().map(|t| &t.employee).any(|e| self.employee.matches_view(e)) &&
		self.time_begin.set_matches(&timesheets.iter().map(|t| &t.time_begin).collect()) &&
		self.time_end.set_matches(&timesheets.iter().map(|t| &t.time_end).collect())
	}

	/// # Summary
	///
	/// Return `true` if `timesheet` is a match.
	pub fn set_matches(&self, timesheets: &[clinvoice_data::Timesheet]) -> bool
	{
		self.employee.id.set_matches(&timesheets.iter().map(|t| &t.employee_id).collect()) &&
		self.time_begin.set_matches(&timesheets.iter().map(|t| &t.time_begin).collect()) &&
		self.time_end.set_matches(&timesheets.iter().map(|t| &t.time_end).collect())
	}
}

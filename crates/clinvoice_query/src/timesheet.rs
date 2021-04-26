use
{
	super::{Employee, Expense, Match, MatchStr, Result},

	clinvoice_data::
	{
		chrono::{DateTime, Local},
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
	pub expenses: Expense<'m>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub time_begin: Match<'m, DateTime<Local>>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub time_end: Match<'m, Option<DateTime<Local>>>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub work_notes: MatchStr<String>,
}

impl Timesheet<'_>
{
	/// # Summary
	///
	/// Return `true` if `timesheet` is a match.
	pub fn set_matches<'item>(&self, mut timesheets: impl Iterator<Item=&'item clinvoice_data::Timesheet>) -> Result<bool>
	{
		Ok(
			self.employee.id.set_matches(&timesheets.by_ref().map(|t| &t.employee_id).collect()) &&
			self.expenses.set_matches(timesheets.by_ref().map(|t| &t.expenses).flatten())? &&
			self.time_begin.set_matches(&timesheets.by_ref().map(|t| DateTime::from(t.time_begin)).collect::<Vec<_>>().iter().collect()) &&
			self.time_end.set_matches(&timesheets.by_ref().map(|t| t.time_end.map(DateTime::from)).collect::<Vec<_>>().iter().collect()) &&
			self.work_notes.set_matches(timesheets.map(|t| t.work_notes.as_ref()))?
		)
	}

	/// # Summary
	///
	/// Return `true` if `timesheet` is a match.
	pub fn set_matches_view<'item>(&self, mut timesheets: impl Iterator<Item=&'item TimesheetView>) -> Result<bool>
	{
		Ok(
			self.employee.set_matches_view(timesheets.by_ref().map(|t| &t.employee))? &&
			self.expenses.set_matches(timesheets.by_ref().map(|t| &t.expenses).flatten())? &&
			self.time_begin.set_matches(&timesheets.by_ref().map(|t| DateTime::from(t.time_begin)).collect::<Vec<_>>().iter().collect()) &&
			self.time_end.set_matches(&timesheets.by_ref().map(|t| t.time_end.map(DateTime::from)).collect::<Vec<_>>().iter().collect()) &&
			self.work_notes.set_matches(timesheets.map(|t| t.work_notes.as_ref()))?
		)
	}
}

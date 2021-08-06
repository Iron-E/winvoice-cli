use clinvoice_data::{chrono::NaiveDateTime, views::TimesheetView};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Employee, Expense, Match, MatchStr, Result};

/// # Summary
///
/// An [`Timesheet`](clinvoice_data::Timesheet) with [matchable](Match) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Timesheet<'m>
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub employee: Employee<'m>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub expenses: Expense<'m>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub time_begin: Match<'m, NaiveDateTime>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub time_end: Match<'m, Option<NaiveDateTime>>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub work_notes: MatchStr<String>,
}

impl Timesheet<'_>
{
	/// # Summary
	///
	/// Return `true` if `timesheet` is a match.
	pub fn set_matches<'item>(
		&self,
		timesheets: &mut impl Iterator<Item = &'item clinvoice_data::Timesheet>,
	) -> Result<bool>
	{
		Ok(self
			.employee
			.id
			.set_matches(&timesheets.by_ref().map(|t| &t.employee_id).collect()) &&
			self
				.expenses
				.set_matches(&mut timesheets.by_ref().map(|t| &t.expenses).flatten())? &&
			self.time_begin.set_matches(
				&timesheets
					.by_ref()
					.map(|t| t.time_begin.naive_local())
					.collect::<Vec<_>>()
					.iter()
					.collect(),
			) && self.time_end.set_matches(
			&timesheets
				.by_ref()
				.map(|t| t.time_end.map(|time| time.naive_local()))
				.collect::<Vec<_>>()
				.iter()
				.collect(),
		) && self
			.work_notes
			.set_matches(&mut timesheets.map(|t| t.work_notes.as_ref()))?)
	}

	/// # Summary
	///
	/// Return `true` if `timesheet` is a match.
	pub fn set_matches_view<'item>(
		&self,
		timesheets: &mut impl Iterator<Item = &'item TimesheetView>,
	) -> Result<bool>
	{
		Ok(self
			.employee
			.set_matches_view(&mut timesheets.by_ref().map(|t| &t.employee))? &&
			self
				.expenses
				.set_matches(&mut timesheets.by_ref().map(|t| &t.expenses).flatten())? &&
			self.time_begin.set_matches(
				&timesheets
					.by_ref()
					.map(|t| t.time_begin.naive_local())
					.collect::<Vec<_>>()
					.iter()
					.collect(),
			) && self.time_end.set_matches(
			&timesheets
				.by_ref()
				.map(|t| t.time_end.map(|time| time.naive_local()))
				.collect::<Vec<_>>()
				.iter()
				.collect(),
		) && self
			.work_notes
			.set_matches(&mut timesheets.map(|t| t.work_notes.as_ref()))?)
	}
}

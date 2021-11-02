use clinvoice_schema::chrono::NaiveDateTime;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Employee, Expense, Job, Match, MatchStr};

/// # Summary
///
/// An [`Timesheet`](clinvoice_schema::Timesheet) with [matchable](Match) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Timesheet<'m>
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub employee: Employee<'m>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub expenses: Expense<'m>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub job: Job<'m>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub time_begin: Match<'m, NaiveDateTime>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub time_end: Match<'m, Option<NaiveDateTime>>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub work_notes: MatchStr<String>,
}

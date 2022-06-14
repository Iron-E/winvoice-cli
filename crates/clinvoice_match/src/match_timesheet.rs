mod exchangeable;

use clinvoice_schema::{chrono::NaiveDateTime, Id};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchEmployee, MatchExpense, MatchJob, MatchSet, MatchStr};
use crate::MatchRow;

/// # Summary
///
/// An [`Timesheet`](clinvoice_schema::Timesheet) with [matchable](Match) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct MatchTimesheet
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub id: Match<Id>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub employee: MatchRow<MatchEmployee>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub expenses: MatchSet<MatchRow<MatchExpense>>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub job: MatchRow<MatchJob>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub time_begin: Match<NaiveDateTime>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub time_end: Match<Option<NaiveDateTime>>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub work_notes: MatchStr<String>,
}

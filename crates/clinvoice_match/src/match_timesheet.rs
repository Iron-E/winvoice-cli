use std::borrow::Cow;

use clinvoice_schema::chrono::NaiveDateTime;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchEmployee, MatchExpense, MatchJob, MatchStr};

/// # Summary
///
/// An [`Timesheet`](clinvoice_schema::Timesheet) with [matchable](Match) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct MatchTimesheet<'m>
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub employee: MatchEmployee<'m>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub expenses: MatchExpense<'m>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub job: MatchJob<'m>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub time_begin: Match<'m, NaiveDateTime>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub time_end: Match<'m, Option<NaiveDateTime>>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub work_notes: MatchStr<Cow<'m, str>>,
}

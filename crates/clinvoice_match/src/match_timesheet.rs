mod exchangeable;

use clinvoice_schema::{chrono::NaiveDateTime, Id};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchEmployee, MatchExpense, MatchJob, MatchSet, MatchStr};
use crate::MatchOption;

/// A [`Timesheet`](clinvoice_schema::Timesheet) with [matchable](clinvoice_match) fields.
///
/// [`MatchTimesheet`] matches IFF all of its fields also match.
///
/// # Examples
///
/// ## YAML
///
/// Requires the `serde_support` feature. If any field is omitted, it will be set to the
/// [`Default`] for its type.
///
/// ```rust
/// # assert!(serde_yaml::from_str::<clinvoice_match::MatchTimesheet>(r#"
/// id: any
/// employee:
///   name:
///     regex: '^[JR]on$'
/// expenses:
///   contains:
///     category:
///       equal_to: "Travel"
/// job:
///   client:
///     name:
///       contains: "Interational"
/// time_begin:
///   less_than: "2022-01-01T00:00:00"
/// time_end: none
/// work_notes: any
/// # "#).is_ok());
/// ```
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MatchTimesheet
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub id: Match<Id>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub employee: MatchEmployee,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub expenses: MatchSet<MatchExpense>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub job: MatchJob,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub time_begin: Match<NaiveDateTime>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub time_end: MatchOption<NaiveDateTime>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub work_notes: MatchStr<String>,
}

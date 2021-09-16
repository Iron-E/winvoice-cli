use clinvoice_data::{chrono::NaiveDateTime, Id};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Invoice, Match, MatchStr, Organization, Timesheet};

/// # Summary
///
/// An [`Job`](clinvoice_data::Job) with [matchable](Match) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Job<'m>
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub client: Organization<'m>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub date_close: Match<'m, Option<NaiveDateTime>>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub date_open: Match<'m, NaiveDateTime>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub id: Match<'m, Id>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub invoice: Invoice<'m>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub notes: MatchStr<String>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub objectives: MatchStr<String>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub timesheets: Timesheet<'m>,
}

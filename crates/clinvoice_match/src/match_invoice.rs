mod exchangeable;

use clinvoice_schema::{chrono::NaiveDateTime, Money};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::Match;

/// # Summary
///
/// An [`Invoice`](clinvoice_schema::Invoice) with [matchable](Match) fields.
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct MatchInvoice
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub date_issued: Match<Option<NaiveDateTime>>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub date_paid: Match<Option<NaiveDateTime>>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub hourly_rate: Match<Money>,
}

use clinvoice_data::{chrono::NaiveDateTime, Money};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::Match;

/// # Summary
///
/// An [`Invoice`](clinvoice_data::Invoice) with [matchable](Match) fields.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Invoice<'m>
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub issued: Match<'m, Option<NaiveDateTime>>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub paid: Match<'m, Option<NaiveDateTime>>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub hourly_rate: Match<'m, Money>,
}

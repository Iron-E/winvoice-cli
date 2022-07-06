#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::MatchStr;
use crate::MatchLocation;

/// # Summary
///
/// A [`Contact`](clinvoice_schema::Contact) with [matchable](Match) fields.
#[cfg_attr(
	feature = "serde_support",
	derive(Deserialize, Serialize),
	serde(rename_all = "snake_case")
)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum MatchContactKind
{
	/// Same as [`ContactKind::Address`](clinvoice_schema::ContactKind::Address).
	Address(#[cfg_attr(feature = "serde_support", serde(default))] MatchLocation),

	/// Same as [`Always`](crate::Match::Always).
	#[default]
	Any,

	/// Same as [`ContactKind::Email`](clinvoice_schema::ContactKind::Email).
	Email(#[cfg_attr(feature = "serde_support", serde(default))] MatchStr<String>),

	/// Same as [`ContactKind::Other`](clinvoice_schema::ContactKind::Other).
	Other(#[cfg_attr(feature = "serde_support", serde(default))] MatchStr<String>),

	/// Same as [`ContactKind::Phone`](clinvoice_schema::ContactKind::Phone).
	Phone(#[cfg_attr(feature = "serde_support", serde(default))] MatchStr<String>),
}

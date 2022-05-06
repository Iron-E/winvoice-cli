mod default;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::MatchStr;
use crate::MatchLocation;

/// # Summary
///
/// A [`Contact`](clinvoice_schema::Contact) with [matchable](Match) fields.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub enum MatchContactKind
{
	/// Same as [`Always`](crate::Match::Always).
	Always,

	/// Same as [`Contact::Address`](clinvoice_schema::Contact::Address).
	SomeAddress(#[cfg_attr(feature = "serde_support", serde(default))] MatchLocation),

	/// Same as [`Contact::Email`](clinvoice_schema::Contact::Email).
	SomeEmail(#[cfg_attr(feature = "serde_support", serde(default))] MatchStr<String>),

	/// Same as [`Contact::Phone`](clinvoice_schema::Contact::Phone).
	SomePhone(#[cfg_attr(feature = "serde_support", serde(default))] MatchStr<String>),
}

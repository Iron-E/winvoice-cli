mod default;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use crate::MatchLocation;

use super::{Match, MatchStr};

/// # Summary
///
/// A [`Contact`](clinvoice_schema::Contact) with [matchable](Match) fields.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub enum MatchContact
{
	/// Same as [`Any`](crate::Match::Any).
	Any,

	/// Same as [`None`](crate::MatchOuterLocation::None).
	None,

	/// Same as [`Contact::Address`](clinvoice_schema::Contact::Address).
	SomeAddress
	{
		#[cfg_attr(feature = "serde_support", serde(default))]
		location: MatchLocation,

		#[cfg_attr(feature = "serde_support", serde(default))]
		export: Match<bool>,
	},

	/// Same as [`Contact::Email`](clinvoice_schema::Contact::Email).
	SomeEmail
	{
		#[cfg_attr(feature = "serde_support", serde(default))]
		email: MatchStr<String>,

		#[cfg_attr(feature = "serde_support", serde(default))]
		export: Match<bool>,
	},

	/// Same as [`Contact::Phone`](clinvoice_schema::Contact::Phone).
	SomePhone
	{
		#[cfg_attr(feature = "serde_support", serde(default))]
		phone: MatchStr<String>,

		#[cfg_attr(feature = "serde_support", serde(default))]
		export: Match<bool>,
	},
}

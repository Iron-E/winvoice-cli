mod match_contact_kind;

pub use match_contact_kind::MatchContactKind;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchStr};

/// # Summary
///
/// An [`Contact`](clinvoice_schema::Contact) with [matchable](Match) fields.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct MatchContact
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub label: MatchStr<String>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub export: Match<bool>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub kind: MatchContactKind,
}

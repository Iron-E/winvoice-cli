mod match_contact_kind;

pub use match_contact_kind::MatchContactKind;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::MatchStr;

/// # Summary
///
/// An [`Contact`](clinvoice_schema::Contact) with [matchable](Match) fields.
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct MatchContact
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub kind: MatchContactKind,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub label: MatchStr<String>,
}

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::MatchStr;
use crate::MatchLocation;

/// A [`ContactKind`](clinvoice_schema::ContactKind) with [matchable](clinvoice_match) fields.
///
/// [`MatchContact`] matches IFF its variant matches.
///
/// # Examples
///
/// This is an example for how a [`MatchContactKind`] may look as YAML (requires the
/// `serde_support` feature):
///
/// ```rust
/// use clinvoice_match::MatchContactKind;
/// use serde_yaml::from_str;
///
/// assert!(from_str::<MatchContactKind>("
///   email:
///     equal_to: 'foo@bar.io'
/// ").is_ok());
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

	/// Always match.
	#[default]
	Any,

	/// Same as [`ContactKind::Email`](clinvoice_schema::ContactKind::Email).
	Email(#[cfg_attr(feature = "serde_support", serde(default))] MatchStr<String>),

	/// Same as [`ContactKind::Other`](clinvoice_schema::ContactKind::Other).
	Other(#[cfg_attr(feature = "serde_support", serde(default))] MatchStr<String>),

	/// Same as [`ContactKind::Phone`](clinvoice_schema::ContactKind::Phone).
	Phone(#[cfg_attr(feature = "serde_support", serde(default))] MatchStr<String>),
}

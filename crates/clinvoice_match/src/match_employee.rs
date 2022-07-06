use clinvoice_schema::Id;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchStr};

/// A [`Employee`](clinvoice_schema::Employee) with [matchable](clinvoice_match) fields.
///
/// [`MatchEmployee`] matches IFF all of its fields also match.
///
/// # Examples
///
/// ## YAML
///
/// Requires the `serde_support` feature. If any field is omitted, it will be set to the
/// [`Default`] for its type.
///
/// ```rust
/// # assert!(serde_yaml::from_str::<clinvoice_match::MatchEmployee>(r#"
/// id: any
/// name:
///   regex: 'son\b'
/// status:
///   equal_to: "Hired"
/// title:
///   contains: "C"
/// # "#).is_ok());
/// ```
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct MatchEmployee
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub id: Match<Id>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub name: MatchStr<String>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub status: MatchStr<String>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub title: MatchStr<String>,
}

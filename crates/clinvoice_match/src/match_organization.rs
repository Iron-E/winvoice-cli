use clinvoice_schema::Id;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchLocation, MatchStr};

/// A [`Organization`](clinvoice_schema::Organization) with [matchable](clinvoice_match) fields.
///
/// [`MatchOrganization`] matches IFF all of its fields also match.
///
/// # Examples
///
/// ## YAML
///
/// Requires the `serde_support` feature. If any field is omitted, it will be set to the
/// [`Default`] for its type.
///
/// ```rust
/// # assert!(serde_yaml::from_str::<clinvoice_match::MatchOrganization>(r#"
/// id: any
/// location:
///   outer:
///     some:
///       name:
///         equal_to: "Mexico"
/// name:
///   equal_to: "Some Company"
/// # "#).is_ok());
/// ```
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MatchOrganization
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub id: Match<Id>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub location: MatchLocation,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub name: MatchStr<String>,
}

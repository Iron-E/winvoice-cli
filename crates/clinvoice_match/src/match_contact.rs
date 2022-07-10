mod match_contact_kind;

pub use match_contact_kind::MatchContactKind;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::MatchStr;

/// A [`Contact`](clinvoice_schema::Contact) with [matchable](clinvoice_match) fields.
///
/// [`MatchContact`] matches IFF all of its fields also match.
///
/// # Examples
///
/// ## YAML
///
/// Requires the `serde_support` feature. If any field is omitted, it will be set to the
/// [`Default`] for its type.
///
/// ```rust
/// # assert!(serde_yaml::from_str::<clinvoice_match::MatchContact>(r#"
/// kind:
///   email:
///     equal_to: "foo@bar.io"
/// label:
///   equal_to: "Primary Email"
/// # "#).is_ok());
/// ```
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct MatchContact
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub kind: MatchContactKind,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub label: MatchStr<String>,
}

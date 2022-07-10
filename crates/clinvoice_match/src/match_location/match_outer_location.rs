#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::MatchLocation;

/// An [`Option<Location>`] with [matchable](clinvoice_match) fields.
///
/// [`MatchOuterLocation`] matches IFF its variant matches.
///
/// # Examples
///
/// ## YAML
///
/// Requires the `serde_support` feature. If any field is omitted, it will be set to the
/// [`Default`] for its type.
///
/// ```rust
/// # use clinvoice_match::MatchOuterLocation;
/// # use serde_yaml::from_str;
/// # assert!(from_str::<MatchOuterLocation>("
/// any
/// # ").is_ok());
///
/// // ------------------
///
/// # assert!(from_str::<MatchOuterLocation>("
/// none
/// # ").is_ok());
///
/// // ------------------
///
/// # assert!(from_str::<MatchOuterLocation>(r#"
/// some:
///   name:
///     equal_to: "Antarctica"
/// # "#).is_ok());
/// ```
#[cfg_attr(
	feature = "serde_support",
	derive(Deserialize, Serialize),
	serde(rename_all = "snake_case")
)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum MatchOuterLocation
{
	/// Always match.
	#[default]
	Any,

	/// Match IFF the [`Location`](clinvoice_schema::Location)'s `outer` field
	/// [`is_none`](Option::is_none).
	None,

	/// Match IFF the [`Location`](clinvoice_schema::Location)'s `outer` field
	/// [`is_some`](Option::is_some) and matches the contained [`MatchLocation`].
	///
	/// TODO: [flatten this](https://github.com/serde-rs/serde/issues/1402)
	Some(Box<MatchLocation>),
}

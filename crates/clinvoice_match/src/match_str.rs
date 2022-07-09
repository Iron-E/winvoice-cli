mod default;
mod from;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// A value which describes the condition which some string of type `T` must meet in order to
/// "_match_".
///
/// # Warnings
///
/// * `MatchStr::Not(Box::new(MatchStr::Any))` is always `false` and often begets a runtime
///   [`Error`](std::error::Error).
///
/// # Examples
///
/// This is an example for how a [`MatchStr`] should be interpreted:
///
/// ```rust
/// use clinvoice_match::MatchStr;
/// use regex::Regex;
///
/// fn matches(condition: MatchStr<&str>, x: &str) -> bool {
///   match condition {
///     MatchStr::And(conditions) => conditions.into_iter().all(|c| matches(c, x)),
///     MatchStr::Any => true,
///     MatchStr::Contains(value) => x.contains(value),
///     MatchStr::EqualTo(value) => value == x,
///     MatchStr::Not(c) => !matches(*c, x),
///     MatchStr::Or(conditions) => conditions.into_iter().any(|c| matches(c, x)),
///     MatchStr::Regex(value) => Regex::new(value).unwrap().is_match(x),
///   }
/// }
///
/// assert!(matches(MatchStr::Contains("f"), "foo"));
/// assert!(matches(MatchStr::EqualTo("foo"), "foo"));
/// assert!(matches(MatchStr::Regex("fo{2,}"), "foo"));
/// assert!(matches(
///   MatchStr::Not(Box::new(MatchStr::Or(vec![
///     MatchStr::Contains("b"),
///     MatchStr::Contains("a")
///   ]))),
///   "foo",
/// ));
/// ```
///
/// ## YAML
///
/// Requires the `serde_support` feature.
///
/// ```rust
/// # type MatchStr = clinvoice_match::MatchStr<String>;
/// # use serde_yaml::from_str;
/// # assert!(from_str::<MatchStr>(r#"
/// and:
///   - contains: "f"
///   - regex: 'o{2,}$'
/// # "#).is_ok());
///
/// // -------------------
///
/// # assert!(from_str::<MatchStr>("
/// any
/// # ").is_ok());
///
/// // -------------------
///
/// # assert!(from_str::<MatchStr>(r#"
/// contains: "foo"
/// # "#).is_ok());
///
/// // -------------------
///
/// # assert!(from_str::<MatchStr>(r#"
/// equal_to: "foo"
/// # "#).is_ok());
///
/// // -------------------
///
/// # assert!(from_str::<MatchStr>(r#"
/// not:
///   equal_to: "bar"
/// # "#).is_ok());
///
/// // -------------------
///
/// # assert!(from_str::<MatchStr>(r#"
/// or:
///   - not:
///       contains: "bar"
///   - equal_to: "foobar"
/// # "#).is_ok());
///
/// // -------------------
///
/// # assert!(from_str::<MatchStr>("
/// regex: 'fo{2,}'
/// # ").is_ok());
/// ```
///
/// ### Warnings
///
/// Never use the following, as it is always `false` and often begets an error:
///
/// ```rust
/// # assert!(serde_yaml::from_str::<clinvoice_match::Match<isize>>("
/// not: any
/// # ").is_ok());
/// ```
#[cfg_attr(
	feature = "serde_support",
	derive(Deserialize, Serialize),
	serde(rename_all = "snake_case")
)]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MatchStr<T>
{
	/// Match IFF all contained [`MatchStr`]s also match.
	And(Vec<Self>),

	/// Always match.
	Any,

	/// Match IFF some string `s` is partially equal to the contained value (e.g. "foo" contains
	/// "oo").
	Contains(T),

	/// Match IFF some string `s` matches the contained value.
	EqualTo(T),

	/// Match IFF the contained [`MatchStr`] does _not_ match.
	Not(Box<Self>),

	/// Match IFF any contained [`MatchStr`] matches.
	Or(Vec<Self>),

	/// Match IFF some string `s` is described by this value when interpreted as a regular
	/// expression.
	///
	/// # Warnings
	///
	/// The syntax of a regular expression is highly dependent on the adapter which is being used:
	///
	/// * [Postgres](https://www.postgresql.org/docs/current/functions-matching.html#FUNCTIONS-POSIX-TABLE)
	Regex(T),
}

impl<T> MatchStr<T>
{
	/// Transform some [`MatchStr`] of type `T` into another type `U` by providing a mapping `f`unction.
	///
	/// # See also
	///
	/// * [`Iterator::map`]
	///
	/// # Examples
	///
	/// ```rust
	/// use clinvoice_match::MatchStr;
	/// # use pretty_assertions::assert_eq;
	///
	/// assert_eq!(
	///   MatchStr::EqualTo("5").map(|s| s.to_string()),
	///   MatchStr::EqualTo("5".to_string())
	/// );
	/// ```
	pub fn map<U>(self, f: impl Copy + Fn(T) -> U) -> MatchStr<U>
	{
		match self
		{
			Self::And(match_conditions) =>
			{
				MatchStr::And(match_conditions.into_iter().map(|m| m.map(f)).collect())
			},
			Self::Any => MatchStr::Any,
			Self::Contains(x) => MatchStr::Contains(f(x)),
			Self::EqualTo(x) => MatchStr::EqualTo(f(x)),
			Self::Not(match_condition) => MatchStr::Not(match_condition.map(f).into()),
			Self::Or(match_conditions) =>
			{
				MatchStr::Or(match_conditions.into_iter().map(|m| m.map(f)).collect())
			},
			Self::Regex(x) => MatchStr::Regex(f(x)),
		}
	}

	/// Transform some [`MatchStr`] of type `T` into another type `U` by providing a mapping function.
	///
	/// # See also
	///
	/// * [`MatchStr::map`]
	pub fn map_ref<U>(&self, f: impl Copy + Fn(&T) -> U) -> MatchStr<U>
	{
		match self
		{
			Self::And(match_conditions) =>
			{
				MatchStr::And(match_conditions.iter().map(|m| m.map_ref(f)).collect())
			},
			Self::Any => MatchStr::Any,
			Self::Contains(x) => MatchStr::Contains(f(x)),
			Self::EqualTo(x) => MatchStr::EqualTo(f(x)),
			Self::Not(match_condition) => MatchStr::Not(match_condition.map_ref(f).into()),
			Self::Or(match_conditions) =>
			{
				MatchStr::Or(match_conditions.iter().map(|m| m.map_ref(f)).collect())
			},
			Self::Regex(x) => MatchStr::Regex(f(x)),
		}
	}
}

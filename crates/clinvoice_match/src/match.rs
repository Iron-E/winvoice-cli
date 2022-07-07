mod default;
mod exchangeable;
mod from;

use core::{cmp::Eq, fmt::Debug};

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// A value which describes the condition which some value of type `T` must meet in order to
/// "_match_".
///
/// # Warnings
///
/// * `Match::Not(Box::new(Match::Any))` is always `false` and often begets a runtime
///   [`Error`](std::error::Error).
/// * You should _never_ use [`Match<Option<T>>`]. Instead, use [`MatchOption<T>`](crate::MatchOption).
///
/// # Examples
///
/// This is an example for how a [`Match`] should be interpreted:
///
/// ```rust
/// use clinvoice_match::Match;
///
/// fn matches(condition: Match<isize>, x: isize) -> bool {
///   match condition {
///     Match::And(conditions) => conditions.into_iter().all(|c| matches(c, x)),
///     Match::Any => true,
///     Match::EqualTo(value) => value == x,
///     Match::GreaterThan(value) => x > value,
///     Match::InRange(lower, upper) => lower <= x && x < upper,
///     Match::LessThan(value) => x < value,
///     Match::Not(c) => !matches(*c, x),
///     Match::Or(conditions) => conditions.into_iter().any(|c| matches(c, x)),
///   }
/// }
///
/// assert!(matches(Match::EqualTo(3), 3));
/// assert!(matches(Match::InRange(5, 10), 9));
/// assert!(matches(Match::LessThan(4), 1));
/// assert!(matches(
///   Match::Not(Box::new(Match::Or(vec![
///     Match::GreaterThan(1),
///     Match::LessThan(-1),
///   ]))),
///   0,
/// ));
/// ```
///
/// ## YAML
///
/// Requires the `serde_support` feature.
///
/// ```rust
/// # assert!(serde_yaml::from_str::<clinvoice_match::Match<isize>>("
/// and:
///   - not:
///       equal_to: 3
///   - in_range: [0, 10]
/// # ").is_ok());
/// ```
///
/// ```rust
/// # assert!(serde_yaml::from_str::<clinvoice_match::Match<isize>>("
/// any
/// # ").is_ok());
/// ```
///
/// ```rust
/// # assert!(serde_yaml::from_str::<clinvoice_match::Match<isize>>("
/// equal_to: 3
/// # ").is_ok());
/// ```
///
/// ```rust
/// # assert!(serde_yaml::from_str::<clinvoice_match::Match<isize>>("
/// less_than: 3
/// # ").is_ok());
/// ```
///
/// ```rust
/// # assert!(serde_yaml::from_str::<clinvoice_match::Match<isize>>("
/// greater_than: 3
/// # ").is_ok());
/// ```
///
/// ```rust
/// # assert!(serde_yaml::from_str::<clinvoice_match::Match<isize>>("
/// in_range: [0, 3]
/// # ").is_ok());
/// ```
///
/// ```rust
/// # assert!(serde_yaml::from_str::<clinvoice_match::Match<isize>>("
/// not:
///   equal_to: 3
/// # ").is_ok());
/// ```
///
/// ```rust
/// # assert!(serde_yaml::from_str::<clinvoice_match::Match<isize>>("
/// or:
///   - greater_than: 2
///   - equal_to: 0
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
pub enum Match<T>
{
	/// Match IFF all contained [`Match`]es also match.
	And(Vec<Self>),

	/// Always match.
	Any,

	/// Match IFF some value `v` matches the contained value.
	EqualTo(T),

	/// Match IFF some value `v` is greater than  (`>`) this value.
	GreaterThan(T),

	/// Match IFF some value `v` is greater-than-or-equal-to (`>=`) the left-hand contained value, but is
	/// less than (`<`) the right-hand contained value.
	InRange(T, T),

	/// Match IFF some value `v` is less than  (`>`) this value.
	LessThan(T),

	/// Match IFF the contained [`Match`] does _not_ match.
	Not(Box<Self>),

	/// Match IFF any contained [`Match`] matches.
	Or(Vec<Self>),
}

impl<T> Match<T>
{
	/// Transform some [`Match`] of type `T` into another type `U` by providing a mapping `f`unction.
	///
	/// # See also
	///
	/// * [`Iterator::map`]
	///
	/// # Examples
	///
	/// ```rust
	/// use clinvoice_match::Match;
	///
	/// assert_eq!(
	///   Match::EqualTo("5").map(|s| s.parse::<isize>().unwrap()),
	///   Match::EqualTo(5)
	/// );
	/// ```
	pub fn map<U>(self, f: impl Copy + Fn(T) -> U) -> Match<U>
	{
		match self
		{
			Self::And(match_conditions) =>
			{
				Match::And(match_conditions.into_iter().map(|m| m.map(f)).collect())
			},
			Self::Any => Match::Any,
			Self::EqualTo(x) => Match::EqualTo(f(x)),
			Self::GreaterThan(x) => Match::GreaterThan(f(x)),
			Self::InRange(low, high) => Match::InRange(f(low), f(high)),
			Self::LessThan(x) => Match::LessThan(f(x)),
			Self::Not(match_condition) => Match::Not(match_condition.map(f).into()),
			Self::Or(match_conditions) =>
			{
				Match::Or(match_conditions.into_iter().map(|m| m.map(f)).collect())
			},
		}
	}

	/// Transform some [`Match`] of type `T` into another type `U` by providing a mapping `f`unction.
	///
	/// # See also
	///
	/// * [`Match::map`]
	pub fn map_ref<U>(&self, f: impl Copy + Fn(&T) -> U) -> Match<U>
	{
		match self
		{
			Self::And(match_conditions) =>
			{
				Match::And(match_conditions.iter().map(|m| m.map_ref(f)).collect())
			},
			Self::Any => Match::Any,
			Self::EqualTo(x) => Match::EqualTo(f(x)),
			Self::GreaterThan(x) => Match::GreaterThan(f(x)),
			Self::InRange(low, high) => Match::InRange(f(low), f(high)),
			Self::LessThan(x) => Match::LessThan(f(x)),
			Self::Not(match_condition) => Match::Not(match_condition.map_ref(f).into()),
			Self::Or(match_conditions) =>
			{
				Match::Or(match_conditions.iter().map(|m| m.map_ref(f)).collect())
			},
		}
	}
}

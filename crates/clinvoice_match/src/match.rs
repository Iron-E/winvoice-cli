mod default;
mod exchangeable;
mod from;

use core::{cmp::Eq, fmt::Debug};

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// A value which describes the condition which some value of type `T` must meet in order to
/// "_match_".
///
/// # Examples
///
/// ```rust
/// use clinvoice_match::Match;
///
/// // this is an example for how a match should be interpreted
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
///   Match::Not(Match::Or(vec![Match::GreaterThan(1), Match::LessThan(-1)]).into()),
///   0,
/// ));
/// ```
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
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

	/// Match IFF any contained [`Match`]es also matches.
	Or(Vec<Self>),
}

impl<T> Match<T>
{
	/// Transform some `Match` of type `T` into another type `U` by providing a mapping `f`unction.
	///
	/// # See also
	///
	/// * [`Iterator::map`]
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

	/// Transform some `Match` of type `T` into another type `U` by providing a mapping `f`unction.
	///
	/// # See also
	///
	/// * [`Iterator::map`]
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

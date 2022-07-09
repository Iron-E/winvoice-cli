mod default;
mod exchangeable;
mod from;

use core::{cmp::Eq, fmt::Debug};

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// A value which describes the condition which some
/// [set](https://en.wikipedia.org/wiki/Set_(mathematics)) of type `T` must meet in order to
/// "_match_".
///
/// # Warnings
///
/// * `MatchSet::Not(Box::new(MatchSet::Any))` is always `false` and often begets a runtime
///   [`Error`](std::error::Error).
///
/// # Examples
///
/// This is an example for how a [`MatchSet`] should be interpreted:
///
/// ```rust
/// use std::{collections::HashSet, ops::Deref};
/// use clinvoice_match::{Match, MatchSet};
///
/// fn matches(condition: &Match<isize>, x: isize) -> bool {
///   match condition {
///     Match::And(conditions) => conditions.into_iter().all(|c| matches(c, x)),
///     Match::Any => true,
///     Match::EqualTo(value) => x.eq(value),
///     Match::GreaterThan(value) => x.gt(value),
///     Match::InRange(lower, upper) => lower.le(&x) && x.lt(upper),
///     Match::LessThan(value) => x.le(value),
///     Match::Not(c) => !matches(c.deref(), x),
///     Match::Or(conditions) => conditions.into_iter().any(|c| matches(c, x)),
///   }
/// }
///
/// fn set_matches(condition: &MatchSet<Match<isize>>, set: &HashSet<isize>) -> bool {
///   match condition {
///     MatchSet::And(conditions) => conditions.into_iter().all(|c| set_matches(c, set)),
///     MatchSet::Any => true,
///     MatchSet::Contains(condition) => set.iter().any(|value| matches(condition, *value)),
///     MatchSet::Not(c) => !set_matches(c.deref(), set),
///     MatchSet::Or(conditions) => conditions.into_iter().any(|c| set_matches(c, set)),
///   }
/// }
///
/// let set: HashSet<_> = [1, 3, 5, 7, 9].into_iter().collect();
///
/// assert!(set_matches(
///   &MatchSet::Or(vec![
///     MatchSet::Contains(Match::EqualTo(0)),
///     MatchSet::Contains(Match::GreaterThan(3)),
///   ]),
///   &set,
/// ));
///
/// assert!(set_matches(
///   &MatchSet::Not(Box::new(MatchSet::Contains(Match::InRange(10, 100)))),
///   &set,
/// ));
/// ```
///
/// ## YAML
///
/// Requires the `serde_support` feature.
///
/// ```rust
/// # type MatchSet = clinvoice_match::MatchSet<clinvoice_match::Match<isize>>;
/// # use serde_yaml::from_str;
/// # assert!(from_str::<MatchSet>("
/// and:
///   - contains:
///       equal_to: 5
///   - contains:
///       greater_than: 7
/// # ").is_ok());
///
/// // --------------------
///
/// # assert!(from_str::<MatchSet>("
/// any
/// # ").is_ok());
///
/// // --------------------
///
/// # assert!(from_str::<MatchSet>("
/// contains:
///   in_range: [0, 10]
/// # ").is_ok());
///
/// // --------------------
///
/// # assert!(from_str::<MatchSet>("
/// not:
///   contains:
///     equal_to: 5
/// # ").is_ok());
///
/// // --------------------
///
/// # assert!(from_str::<MatchSet>("
/// or:
///   - contains:
///       equal_to: 5
///   - contains:
///       greater_than: 7
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
pub enum MatchSet<T>
{
	/// Match IFF all contained [`MatchStr`]s also match.
	And(Vec<Self>),

	/// Always match.
	Any,

	/// Match IFF some set contains a value described by this value.
	Contains(T),

	/// Match IFF the contained [`MatchSet`] does _not_ match.
	Not(Box<Self>),

	/// Match IFF any contained [`MatchSet`] matches.
	Or(Vec<Self>),
}

impl<T> MatchSet<T>
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
	/// use clinvoice_match::{Match, MatchSet};
	/// # use pretty_assertions::assert_eq;
	///
	/// assert_eq!(
	///   MatchSet::Contains(Match::EqualTo(7)).map(|_| Match::GreaterThan(5)),
	///   MatchSet::Contains(Match::GreaterThan(5)),
	/// );
	/// ```
	pub fn map<U>(self, f: impl Copy + Fn(T) -> U) -> MatchSet<U>
	{
		match self
		{
			Self::And(match_conditions) =>
			{
				MatchSet::And(match_conditions.into_iter().map(|m| m.map(f)).collect())
			},
			Self::Any => MatchSet::Any,
			Self::Contains(x) => MatchSet::Contains(f(x)),
			Self::Not(match_condition) => MatchSet::Not(match_condition.map(f).into()),
			Self::Or(match_conditions) =>
			{
				MatchSet::Or(match_conditions.into_iter().map(|m| m.map(f)).collect())
			},
		}
	}

	/// Transform some [`Match`] of type `T` into another type `U` by providing a mapping `f`unction.
	///
	/// # See also
	///
	/// * [`Match::map`]
	pub fn map_ref<U>(&self, f: impl Copy + Fn(&T) -> U) -> MatchSet<U>
	{
		match self
		{
			Self::And(match_conditions) =>
			{
				MatchSet::And(match_conditions.iter().map(|m| m.map_ref(f)).collect())
			},
			Self::Any => MatchSet::Any,
			Self::Contains(x) => MatchSet::Contains(f(x)),
			Self::Not(match_condition) => MatchSet::Not(match_condition.map_ref(f).into()),
			Self::Or(match_conditions) =>
			{
				MatchSet::Or(match_conditions.iter().map(|m| m.map_ref(f)).collect())
			},
		}
	}
}

mod default;
mod from;

use core::{cmp::Eq, fmt::Debug};

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// A value which describes the condition which some value of type [`Option<T>`] must meet in order
/// to "_match_".
///
/// # Warnings
///
/// * `MatchOption::Not(Box::new(MatchOption::Any))` is always `false` and often begets a runtime
///   [`Error`](std::error::Error).
///
/// # Notes
///
/// * [`Some(_)`] is equivalent to `MatchOption::Not(MatchOption::None)`.
///
/// # Examples
///
/// This is an example for how a [`MatchOption`] should be interpreted:
///
/// ```rust
/// use clinvoice_match::MatchOption;
///
/// fn matches(condition: MatchOption<isize>, opt_x: Option<isize>) -> bool {
///   match condition {
///     MatchOption::And(conditions) => conditions.into_iter().all(|c| matches(c, opt_x)),
///     MatchOption::Any => true,
///     MatchOption::EqualTo(value) => Some(value) == opt_x,
///     MatchOption::GreaterThan(value) => opt_x.map(|x| x > value).unwrap_or(false),
///     MatchOption::InRange(lower, upper) => opt_x.map(|x| lower <= x && x < upper).unwrap_or(false),
///     MatchOption::LessThan(value) => opt_x.map(|x| x < value).unwrap_or(false),
///     MatchOption::None => opt_x.is_none(),
///     MatchOption::Not(c) => !matches(*c, opt_x),
///     MatchOption::Or(conditions) => conditions.into_iter().any(|c| matches(c, opt_x)),
///   }
/// }
///
/// assert!(matches(MatchOption::Any, None));
/// assert!(matches(MatchOption::Any, Some(1)));
/// assert!(matches(MatchOption::EqualTo(3), Some(3)));
/// assert!(matches(MatchOption::LessThan(4), Some(1)));
/// assert!(matches(MatchOption::None, None));
/// assert!(matches(
///   MatchOption::Not(Box::new(MatchOption::Or(vec![
///     MatchOption::GreaterThan(1),
///     MatchOption::LessThan(-1),
///   ]))),
///   Some(0),
/// ));
/// ```
///
/// ## YAML
///
/// Requires the `serde_support` feature.
///
/// ```rust
/// # type MatchOption = clinvoice_match::MatchOption<isize>;
/// # use serde_yaml::from_str;
/// # assert!(from_str::<MatchOption>("
/// and:
///   - not:
///       equal_to: 3
///   - in_range: [0, 10]
/// # ").is_ok());
///
/// // ----------------------------
///
/// # assert!(from_str::<MatchOption>("
/// any
/// # ").is_ok());
///
/// // ----------------------------
///
/// # assert!(from_str::<MatchOption>("
/// equal_to: 3
/// # ").is_ok());
///
/// // ----------------------------
///
/// # assert!(from_str::<MatchOption>("
/// less_than: 3
/// # ").is_ok());
///
/// // ----------------------------
///
/// # assert!(from_str::<MatchOption>("
/// greater_than: 3
/// # ").is_ok());
///
/// // ----------------------------
///
/// # assert!(from_str::<MatchOption>("
/// in_range: [0, 3]
/// # ").is_ok());
///
/// // ----------------------------
///
/// # assert!(from_str::<MatchOption>("
/// none
/// # ").is_ok());
///
/// // ----------------------------
///
/// # assert!(from_str::<MatchOption>("
/// not: none
/// # ").is_ok());
///
/// // ----------------------------
///
/// # assert!(from_str::<MatchOption>("
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
pub enum MatchOption<T>
{
	/// Match IFF all contained [`MatchOption`]es also match.
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

	/// Match IFF some value `v` is null.
	None,

	/// Match IFF the contained [`MatchOption`] does _not_ match.
	Not(Box<Self>),

	/// Match IFF any contained [`MatchOption`] matches.
	Or(Vec<Self>),
}

impl<T> MatchOption<T>
{
	/// Transform some [`MatchOption`] of type `T` into another type `U` by providing a mapping `f`unction.
	///
	/// # See also
	///
	/// * [`Iterator::map`]
	///
	/// # Examples
	///
	/// ```rust
	/// use clinvoice_match::MatchOption;
	/// # use pretty_assertions::assert_eq;
	///
	/// assert_eq!(
	///   MatchOption::EqualTo("5").map(|s| s.parse::<isize>().unwrap()),
	///   MatchOption::EqualTo(5)
	/// );
	/// ```
	pub fn map<U>(self, f: impl Copy + Fn(T) -> U) -> MatchOption<U>
	{
		match self
		{
			Self::And(match_conditions) =>
			{
				MatchOption::And(match_conditions.into_iter().map(|m| m.map(f)).collect())
			},
			Self::Any => MatchOption::Any,
			Self::EqualTo(x) => MatchOption::EqualTo(f(x)),
			Self::GreaterThan(x) => MatchOption::GreaterThan(f(x)),
			Self::InRange(low, high) => MatchOption::InRange(f(low), f(high)),
			Self::LessThan(x) => MatchOption::LessThan(f(x)),
			Self::None => MatchOption::None,
			Self::Not(match_condition) => MatchOption::Not(match_condition.map(f).into()),
			Self::Or(match_conditions) =>
			{
				MatchOption::Or(match_conditions.into_iter().map(|m| m.map(f)).collect())
			},
		}
	}

	/// Transform some [`MatchOption`] of type `T` into another type `U` by providing a mapping `f`unction.
	///
	/// # See also
	///
	/// * [`MatchOption::map`]
	pub fn map_ref<U>(&self, f: impl Copy + Fn(&T) -> U) -> MatchOption<U>
	{
		match self
		{
			Self::And(match_conditions) =>
			{
				MatchOption::And(match_conditions.iter().map(|m| m.map_ref(f)).collect())
			},
			Self::Any => MatchOption::Any,
			Self::EqualTo(x) => MatchOption::EqualTo(f(x)),
			Self::GreaterThan(x) => MatchOption::GreaterThan(f(x)),
			Self::InRange(low, high) => MatchOption::InRange(f(low), f(high)),
			Self::LessThan(x) => MatchOption::LessThan(f(x)),
			Self::None => MatchOption::None,
			Self::Not(match_condition) => MatchOption::Not(match_condition.map_ref(f).into()),
			Self::Or(match_conditions) =>
			{
				MatchOption::Or(match_conditions.iter().map(|m| m.map_ref(f)).collect())
			},
		}
	}
}

mod default;
mod from;

use core::{cmp::Eq, fmt::Debug};

use clinvoice_finance::ExchangeRates;
use clinvoice_schema::{Currency, Money};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// A value in a retrieval operation.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub enum Match<T>
where
	T: Clone + Debug,
{
	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * `v` equals this value.
	/// * A set of `v`'s type has one element, and is equal to `v`.
	AllGreaterThan(T),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * `v` equals this value.
	/// * A set of `v`'s type has one element, and is equal to `v`.
	AllLessThan(T),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * The value of `v` is greater than or equal to the first value.
	/// * The value of `v` is less than the first value.
	AllInRange(T, T),

	/// # Summary
	///
	/// Match if and only if all of the contained [`Match`]es also match.
	And(Vec<Self>),

	/// # Summary
	///
	/// Always match.
	Any,

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * `v` equals this value.
	/// * A set of `v`'s type has one element, and is equal to `v`.
	EqualTo(T),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * `v` equals this value.
	/// * A set of `v`'s type has one element, and is equal to `v`.
	GreaterThan(T),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * A set of `v` is made up of elements which are contained in this set.
	/// * This set has one element, and `v` is equivalent.
	HasAll(Vec<T>),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * A set of `v`'s type has any value contained in this set.
	/// * `v` is contained within this set.
	HasAny(Vec<T>),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * The value of `v` is greater than or equal to the first value.
	/// * The value of `v` is less than the first value.
	InRange(T, T),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * `v` equals this value.
	/// * A set of `v`'s type has one element, and is equal to `v`.
	LessThan(T),

	/// # Summary
	///
	/// Negate a [`Match`].
	Not(Box<Self>),

	/// # Summary
	///
	/// Match if and only if any of the contained [`Match`]es also match.
	Or(Vec<Self>),
}

impl<T> Match<T>
where
	T: Clone + Debug,
{
	/// # Summary
	///
	/// Transform some `Match` of type `T` into another type `U` by providing a mapping function.
	///
	/// # See also
	///
	/// * [`Iterator::map`]
	pub fn map<U>(self, f: &impl Fn(T) -> U) -> Match<U>
	where
		U: Clone + Debug,
	{
		macro_rules! map {
			($func:ident, $val:ident) => {
				$func($val)
			};
		}

		match self
		{
			Match::AllGreaterThan(x) => Match::AllGreaterThan(map!(f, x)),
			Match::AllInRange(low, high) => Match::AllInRange(map!(f, low), map!(f, high)),
			Match::AllLessThan(x) => Match::AllLessThan(map!(f, x)),
			Match::And(match_conditions) =>
			{
				Match::And(match_conditions.into_iter().map(|m| m.map(f)).collect())
			},
			Match::Any => Match::Any,
			Match::EqualTo(x) => Match::EqualTo(map!(f, x)),
			Match::GreaterThan(x) => Match::GreaterThan(map!(f, x)),
			Match::HasAll(collection) => Match::HasAll(collection.into_iter().map(f).collect()),
			Match::HasAny(collection) => Match::HasAny(collection.into_iter().map(f).collect()),
			Match::InRange(low, high) => Match::InRange(map!(f, low), map!(f, high)),
			Match::LessThan(x) => Match::LessThan(map!(f, x)),
			Match::Not(match_condition) => Match::Not(match_condition.map(f).into()),
			Match::Or(match_conditions) =>
			{
				Match::Or(match_conditions.into_iter().map(|m| m.map(f)).collect())
			},
		}
	}
}

impl Match<Money>
{
	/// # Summary
	///
	/// Exchange a `Match` for an amount of `Money` to another `currency`.
	pub fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Match<Money>
	{
		self.map(&|money| money.exchange(currency, rates))
	}
}

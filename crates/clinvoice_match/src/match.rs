mod default;
mod from;

use core::{cmp::Eq, fmt::Debug, ops::Deref};
use std::borrow::Cow::{self, Owned};

use clinvoice_finance::ExchangeRates;
use clinvoice_schema::{Currency, Money};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// A value in a retrieval operation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub enum Match<'element, T>
where
	T: Clone + Debug,
{
	#[cfg_attr(
		feature = "serde_support",
		serde(bound(deserialize = "T : Deserialize<'de>"))
	)]

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * `v` equals this value.
	/// * A set of `v`'s type has one element, and is equal to `v`.
	AllGreaterThan(Cow<'element, T>),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * `v` equals this value.
	/// * A set of `v`'s type has one element, and is equal to `v`.
	AllLessThan(Cow<'element, T>),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * The value of `v` is greater than or equal to the first value.
	/// * The value of `v` is less than the first value.
	AllInRange(Cow<'element, T>, Cow<'element, T>),

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
	EqualTo(Cow<'element, T>),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * `v` equals this value.
	/// * A set of `v`'s type has one element, and is equal to `v`.
	GreaterThan(Cow<'element, T>),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * A set of `v` is made up of elements which are contained in this set.
	/// * This set has one element, and `v` is equivalent.
	HasAll(Cow<'element, [T]>),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * A set of `v`'s type has any value contained in this set.
	/// * `v` is contained within this set.
	HasAny(Cow<'element, [T]>),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * The value of `v` is greater than or equal to the first value.
	/// * The value of `v` is less than the first value.
	InRange(Cow<'element, T>, Cow<'element, T>),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * `v` equals this value.
	/// * A set of `v`'s type has one element, and is equal to `v`.
	LessThan(Cow<'element, T>),

	/// # Summary
	///
	/// Negate a [`Match`].
	Not(Box<Self>),

	/// # Summary
	///
	/// Match if and only if any of the contained [`Match`]es also match.
	Or(Vec<Self>),
}

impl<'m, T> Match<'m, T>
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
	pub fn map<U>(self, f: &impl Fn(&T) -> U) -> Match<'m, U>
	where
		U: Clone + Debug,
	{
		macro_rules! map {
			($func:ident, $val:ident) => {
				Owned($func($val.deref()))
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

impl<'m> Match<'m, Money>
{
	/// # Summary
	///
	/// Exchange a `Match` for an amount of `Money` to another `currency`.
	pub fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Match<'m, Money>
	{
		self.map(&|money| money.exchange(currency, rates))
	}
}

mod default;
mod from;

use core::{cmp::Eq, fmt::Debug};

use clinvoice_finance::{ExchangeRates, Exchangeable};
use clinvoice_schema::Currency;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// A value in a retrieval operation.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub enum Match<T>
{
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
	/// For some value `v`, match if and only if `v` equals this value.
	EqualTo(T),

	/// # Summary
	///
	/// For some value `v`, match if and only if `v` is greater than this value.
	GreaterThan(T),

	/// # Summary
	///
	/// For some value `v`, match if and only if `v` is greater than or equal to the first value and `v` is less than the second value.
	InRange(T, T),

	/// # Summary
	///
	/// For some value `v`, match if and only if `v` is less than this value.
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
{
	/// # Summary
	///
	/// Transform some `Match` of type `T` into another type `U` by providing a mapping function.
	///
	/// TODO: remove leading borrow from `f` once recursion limit calculation improves
	///
	/// # See also
	///
	/// * [`Iterator::map`]
	pub fn map<U>(self, f: &impl Fn(T) -> U) -> Match<U>
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

	/// # Summary
	///
	/// Transform some `Match` of type `T` into another type `U` by providing a mapping function.
	///
	/// TODO: remove leading borrow from `f` once recursion limit calculation improves
	///
	/// # See also
	///
	/// * [`Iterator::map`]
	pub fn map_ref<U>(&self, f: &impl Fn(&T) -> U) -> Match<U>
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

impl<T> Match<T>
where
	T: Exchangeable,
{
	/// # Summary
	///
	/// Exchange a `Match` for an amount of `Money` to another `currency`.
	pub fn exchange(&self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		self.map_ref(&|e| e.exchange(currency, rates))
	}
}

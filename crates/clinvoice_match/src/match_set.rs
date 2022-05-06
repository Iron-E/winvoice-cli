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
pub enum MatchSet<T>
where
	T: Clone + Debug,
{
	/// # Summary
	///
	/// Always match.
	Always,

	/// # Summary
	///
	/// Match if and only if all of the contained [`Match`]es also match.
	And(Vec<Self>),

	/// # Summary
	///
	/// Match if and only if all of the elements of a set of type `T` contain the elements provided and no others.
	EqualTo(Vec<T>),

	/// # Summary
	///
	/// Match if and only if any of the elements of the provided set are in a given set of type `T`.
	Has(T),

	/// # Summary
	///
	/// Negate a [`Match`].
	Not(Box<Self>),

	/// # Summary
	///
	/// Match if and only if any of the contained [`Match`]es also match.
	Or(Vec<Self>),
}

impl<T> MatchSet<T>
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
	pub fn map<U>(self, f: &impl Fn(T) -> U) -> MatchSet<U>
	where
		U: Clone + Debug,
	{
		match self
		{
			Self::And(match_conditions) =>
			{
				MatchSet::And(match_conditions.into_iter().map(|m| m.map(f)).collect())
			},
			Self::Always => MatchSet::Always,
			Self::EqualTo(set) => MatchSet::EqualTo(set.into_iter().map(f).collect()),
			Self::Has(x) => MatchSet::Has(f(x)),
			Self::Not(match_condition) => MatchSet::Not(match_condition.map(f).into()),
			Self::Or(match_conditions) =>
			{
				MatchSet::Or(match_conditions.into_iter().map(|m| m.map(f)).collect())
			},
		}
	}
}

impl MatchSet<Money>
{
	/// # Summary
	///
	/// Exchange a `Match` for an amount of `Money` to another `currency`.
	pub fn exchange(self, currency: Currency, rates: &ExchangeRates) -> MatchSet<Money>
	{
		self.map(&|money| money.exchange(currency, rates))
	}
}

mod default;
mod exchangeable;
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
pub enum MatchSet<T>
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
	/// Match if and only if any of the elements of the provided set are in a given set of type `T`.
	Contains(T),

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
	pub fn map<U>(self, f: &impl Fn(T) -> U) -> MatchSet<U>
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

	/// # Summary
	///
	/// Transform some `Match` of type `T` into another type `U` by providing a mapping function.
	///
	/// TODO: remove leading borrow from `f` once recursion limit calculation improves
	///
	/// # See also
	///
	/// * [`Iterator::map`]
	pub fn map_ref<U>(&self, f: &impl Fn(&T) -> U) -> MatchSet<U>
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

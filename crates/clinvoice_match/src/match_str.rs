mod default;
mod from;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// A value in a retrieval operation.
#[cfg_attr(
	feature = "serde_support",
	derive(Deserialize, Serialize),
	serde(rename_all = "snake_case"),
)]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MatchStr<T>
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
	/// Match if and only if this value and some other string are exactly the same.
	EqualTo(T),

	/// # Summary
	///
	/// Match if and only if:
	///
	/// * Some string contains this value anywhere in its contents.
	/// * A set of strings contains this value anywhere in its contents.
	Contains(T),

	/// # Summary
	///
	/// Negate a [`Match`].
	Not(Box<Self>),

	/// # Summary
	///
	/// Match if and only if any of the contained [`Match`]es also match.
	Or(Vec<Self>),

	/// # Summary
	///
	/// Match if and only if:
	///
	/// * This regular expression matches some other string.
	/// * This regular expression matches any string in a set of strings.
	///
	/// # Remarks
	///
	/// The regular expression syntax depends on the database adapter:
	///
	/// * [Postgres](https://www.postgresql.org/docs/current/functions-matching.html#FUNCTIONS-POSIX-TABLE)
	Regex(T),
}

impl<T> MatchStr<T>
{
	/// # Summary
	///
	/// Transform some `Match` of type `T` into another type `U` by providing a mapping function.
	///
	/// # See also
	///
	/// * [`Iterator::map`]
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

	/// # Summary
	///
	/// Transform some `Match` of type `T` into another type `U` by providing a mapping function.
	///
	/// # See also
	///
	/// * [`Iterator::map`]
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

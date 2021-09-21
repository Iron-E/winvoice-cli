mod default;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// A value in a retrieval operation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde_support", serde(content = "value", tag = "condition"))]
pub enum MatchStr<S>
where
	S: AsRef<str>,
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
	EqualTo(S),

	/// # Summary
	///
	/// Match if and only if:
	///
	/// * Some string contains this value anywhere in its contents.
	/// * A set of strings contains this value anywhere in its contents.
	Contains(S),

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
	Regex(S),
}

mod display;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// The state of an SQL query as it is being generated.
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WriteContext
{
	/// After any number of `WHERE` conditions, but before a post-`WHERE` condition such as `GROUP
	/// BY`.
	///
	/// # Example
	///
	/// ```sql
	/// SELECT * FROM foo WHERE bar = 7 -- ← a valid SQL query, but would also be valid with another `WHERE` condition
	/// ```
	AcceptingAnotherWhereCondition,

	/// Before any `WHERE` keyword.
	///
	/// # Example
	///
	/// ```sql
	/// SELECT * FROM foo -- ← no `WHERE` yet
	/// ```
	#[default]
	BeforeWhereClause,

	/// In the middle of writing a valid `WHERE` clause. The query needs more content to be
	/// considered valid syntax.
	///
	/// # Example
	///
	/// ```sql
	/// SELECT * FROM foo WHERE bar <= 7 AND -- ← invalid SQL; the current `WHERE` condition is not complete.
	/// ```
	InWhereCondition,
}

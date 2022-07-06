mod display;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// Where inside of the `WHERE` clause this write is taking place.
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WriteContext
{
	/// # Summary
	///
	/// After any number of `WHERE` conditions, but before a post-`WHERE` condition such as `GROUP
	/// BY`.
	///
	/// # Example
	///
	/// ```sql
	/// SELECT * FROM foo WHERE bar = 7 -- ← a valid SQL query, but would also be valid with another `WHERE` condition
	/// ```
	AcceptingAnotherWhereCondition,

	/// # Summary
	///
	/// Before any `WHERE` keyword is written. After a `FROM` or `JOIN` clause.
	///
	/// # Example
	///
	/// ```sql
	/// SELECT * FROM foo -- ← no `WHERE` yet
	/// ```
	#[default]
	BeforeWhereClause,

	/// # Summary
	///
	/// In the middle of writing a complete `WHERE` clause. The query is usually not valid syntax
	/// yet.
	///
	/// # Example
	///
	/// ```sql
	/// SELECT * FROM foo WHERE bar <= 7 AND -- ← invalid SQL; the current `WHERE` condition is not complete.
	/// ```
	InWhereCondition,
}

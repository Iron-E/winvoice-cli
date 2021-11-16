mod from_bool;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// Where inside of the `WHERE` clause this write is taking place.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub enum WriteContext
{
	/// # Summary
	///
	/// Before any `WHERE` keyword is written. After a `FROM` or `JOIN` clause.
	///
	/// # Example
	///
	/// ```ignore
	/// SELECT * FROM foo
	/// ```
	BeforeClause,

	/// # Summary
	///
	/// In the middle of writing a complete `WHERE` clause. Syntax is usually invalid at this point.
	///
	/// # Example
	///
	/// ```ignore
	/// SELECT * FROM foo WHERE bar <= 7 AND
	/// ```
	InsideClause,

	/// # Summary
	///
	/// After the `WHERE` keyword is written.
	///
	/// # Example
	///
	/// ```ignore
	/// SELECT * FROM foo WHERE bar = 7
	/// ```
	AfterClause,
}

impl WriteContext
{
	pub const fn get_prefix(&self) -> &'static str
	{
		match self
		{
			WriteContext::AfterClause => " AND",
			WriteContext::BeforeClause => " WHERE",
			WriteContext::InsideClause => "",
		}
	}
}

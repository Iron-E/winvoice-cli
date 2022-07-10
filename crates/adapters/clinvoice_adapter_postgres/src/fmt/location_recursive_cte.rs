mod display;

use core::fmt::Display;

use clinvoice_adapter::fmt::SnakeCase;
use clinvoice_match::{MatchLocation, MatchOuterLocation};

/// Able to generate [`Display`] impls which are viable for use within a recursive Common Table
/// Expression.
///
/// When generating SQL in such a scenario, it is not known how many identifiers will need to be
/// created. As such, it must be guaranteed that each successive generated identifier is unique
/// from the last. This struct can do, and without the overhead of having to track all of the other
/// identifiers in a [`HashSet`](std::collections::HashSet) (or some similar construct).
///
/// # Examples
///
/// * See [`PgLocation::query_with_recursive`].
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct PgLocationRecursiveCte<T, TOuter>(SnakeCase<T, TOuter>)
where
	T: Display,
	TOuter: Display;

impl PgLocationRecursiveCte<&'static str, &'static str>
{
	/// Determine what the final identifier would be in a recursive CTE based on some [`MatchLocation`].
	pub(crate) const fn from(match_condition: &MatchLocation) -> Self
	{
		match match_condition.outer
		{
			MatchOuterLocation::Some(_) => Self::report(),
			_ => Self::new(),
		}
	}

	/// Create a new identifier for use at the start of a recursive CTE.
	pub(crate) const fn new() -> Self
	{
		PgLocationRecursiveCte(SnakeCase::Head("location"))
	}

	/// Create a new identifier for use at the end of a recursive CTE which has at least one other
	/// identifier.
	pub(crate) const fn report() -> Self
	{
		PgLocationRecursiveCte(SnakeCase::Body("location", "report"))
	}
}

impl<T> PgLocationRecursiveCte<T, &'static str>
where
	T: Display,
{
	/// Get the [`PgLocationRecursiveCte`] representing the identifier which is valid for the next
	/// identifier in a recursive CTE.
	pub(crate) const fn outer(t: T) -> Self
	{
		PgLocationRecursiveCte(SnakeCase::Body(t, "outer"))
	}
}

impl<T, TOuter> PgLocationRecursiveCte<T, TOuter>
where
	T: Display,
	TOuter: Display,
{
	/// See [`SnakeCase::slice_end`]
	pub(crate) const fn slice_end(&self) -> Option<(&T, &TOuter)>
	{
		self.0.slice_end()
	}
}

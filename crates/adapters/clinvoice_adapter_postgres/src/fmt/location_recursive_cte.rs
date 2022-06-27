use core::fmt::Display;

use clinvoice_adapter::fmt::SnakeCase;
use clinvoice_match::{MatchLocation, MatchOuterLocation};

/// # Summary
///
/// Wraps [`Display`] impls  to provide the necessary [`Display`] impl for a recursive Common Table
/// Expression.
///
/// Created to avoid using `format!` every time this pattern was required, thus eagerly allocating
/// a [`String`] even if it was only needed for pushing to another [`String`].
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct PgLocationRecursiveCte;

impl PgLocationRecursiveCte
{
	pub(crate) const fn from(
		match_condition: &MatchLocation,
	) -> SnakeCase<&'static str, &'static str>
	{
		match match_condition.outer
		{
			MatchOuterLocation::Some(_) => Self::report(),
			_ => Self::new(),
		}
	}

	/// # Summary
	///
	/// Create a new recursive CTE identifier for a [`PgLocation`].
	#[allow(clippy::new_ret_no_self)]
	pub(crate) const fn new() -> SnakeCase<&'static str, &'static str>
	{
		SnakeCase::Head("location")
	}

	/// # Summary
	///
	/// Get the [`PgLocationRecursiveCte`] representing the [`Location`](clinvoice_schema::Location) this one.
	pub(crate) const fn outer<T>(t: T) -> SnakeCase<T, &'static str>
	where
		T: Display,
	{
		SnakeCase::Body(t, "outer")
	}

	/// # Summary
	///
	/// The ident used to refer to the rows matching some [`MatchLocation`] at the end of a `WITH
	/// RECURSIVE`.
	pub(crate) const fn report() -> SnakeCase<&'static str, &'static str>
	{
		SnakeCase::Body("location", "report")
	}
}

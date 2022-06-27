mod display;

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
pub(crate) struct PgLocationRecursiveCte<T, TOuter>(SnakeCase<T, TOuter>)
where
	T: Display,
	TOuter: Display;

impl PgLocationRecursiveCte<&'static str, &'static str>
{
	pub(crate) const fn from(match_condition: &MatchLocation) -> Self
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
	pub(crate) const fn new() -> Self
	{
		PgLocationRecursiveCte(SnakeCase::Head("location"))
	}

	/// # Summary
	///
	/// The ident used to refer to the rows matching some [`MatchLocation`] at the end of a `WITH
	/// RECURSIVE`.
	pub(crate) const fn report() -> Self
	{
		PgLocationRecursiveCte(SnakeCase::Body("location", "report"))
	}
}

impl<T> PgLocationRecursiveCte<T, &'static str>
where
	T: Display,
{
	/// # Summary
	///
	/// Get the [`PgLocationRecursiveCte`] representing the [`Location`](clinvoice_schema::Location) this one.
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
	/// # Summary
	///
	/// See [`SnakeCase::slice_end`]
	pub(crate) const fn slice_end(&self) -> Option<(&T, &TOuter)>
	{
		self.0.slice_end()
	}
}

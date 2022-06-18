mod display;

use core::fmt::Display;

use clinvoice_adapter::fmt::SnakeCase;

/// # Summary
///
/// Wraps [`Display`] impls  to provide the necessary [`Display`] impl for a recursive Common Table
/// Expression.
///
/// Created to avoid using `format!` every time this pattern was required, thus eagerly allocating
/// a [`String`] even if it was only needed for pushing to another [`String`].
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct PgLocationRecursiveCte<TCurrent, TPrev>(SnakeCase<TPrev, TCurrent>)
where
	TCurrent: Display,
	TPrev: Display;

impl<TCurrent, TPrev> PgLocationRecursiveCte<TCurrent, TPrev>
where
	TCurrent: Display,
	TPrev: Display,
{
	/// # Summary
	///
	/// Return the previous occurance of the [`PgLocationRecursiveCte`], if there is one.
	pub(crate) const fn prev(&self) -> Option<&TPrev>
	{
		if let Some((left, _)) = self.0.slice_end()
		{
			return Some(left);
		}

		None
	}

	/// # Summary
	///
	/// Get the [`PgLocationRecursiveCte`] representing the [`Location`](clinvoice_schema::Location) this one.
	pub(crate) fn outer(
		self,
	) -> PgLocationRecursiveCte<&'static str, SnakeCase<TPrev, TCurrent>>
	{
		PgLocationRecursiveCte(self.0.push("outer"))
	}
}

impl PgLocationRecursiveCte<&'static str, &'static str>
{
	/// # Summary
	///
	/// Create a new recursive CTE identifier for a [`PgLocation`].
	pub(crate) const fn new() -> Self
	{
		Self(SnakeCase::new("location"))
	}
}

impl PgLocationRecursiveCte<&'static str, SnakeCase<&'static str, &'static str>>
{
	/// # Summary
	///
	/// The ident used to refer to the rows matching some [`MatchLocation`] at the end of a `WITH
	/// RECURSIVE`.
	pub(crate) const fn report() -> Self
	{
		Self(PgLocationRecursiveCte::new().0.push("report"))
	}
}

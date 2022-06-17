mod display;

use core::fmt::Display;

/// # Summary
///
/// Wraps [`Display`] impls  to provide the necessary [`Display`] impl for a recursive Common Table
/// Expression.
///
/// Created to avoid using `format!` every time this pattern was required, thus eagerly allocating
/// a [`String`] even if it was only needed for pushing to another [`String`].
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct PgLocationRecursiveCte<TCurrent, TInner>
where
	TCurrent: Display,
	TInner: Display,
{
	/// # Summary
	///
	/// The name of the current location ident. When combined with `inner` (separated by an
	/// underscore) it should produce the full ident for the current location.
	///
	/// # Example
	///
	/// ```ignore
	/// // prints as "location";
	/// let innermost = PgOuterLocation { inner: "", outer: "location" };
	///
	/// // prints as "location_outer";
	/// let outer = PgOuterLocation { inner: innermost, outer: "outer" };
	///
	/// // prints as "location_outer_outer";
	/// let outer_outer = PgOuterLocation { inner: outer, outer: "outer" };
	/// ```
	///
	/// # See
	///
	/// [`PgOuterLocation::outer`]
	current: TCurrent,

	/// # Summary
	///
	/// The previous [`PgOuterLocation`], or [`None`] if this is the innermost.
	///
	/// # See
	///
	/// [`PgOuterLocation::innermost`]
	inner: Option<TInner>,
}

impl<TCurrent, TInner> PgLocationRecursiveCte<TCurrent, TInner>
where
	TCurrent: Display,
	TInner: Display,
{
	/// # Summary
	///
	/// Get `self.inner`.
	pub(crate) const fn inner(&self) -> Option<&TInner>
	{
		self.inner.as_ref()
	}

	/// # Summary
	///
	/// Get the [`PgOuterLocation`] representing the [`Location`](clinvoice_schema::Location) outer
	/// this one.
	pub(crate) const fn outer(self) -> PgLocationRecursiveCte<&'static str, Self>
	{
		PgLocationRecursiveCte {
			current: "outer",
			inner: Some(self),
		}
	}
}

impl PgLocationRecursiveCte<&'static str, &'static str>
{
	pub(crate) const fn innermost() -> Self
	{
		Self {
			inner: None,
			current: "location",
		}
	}

	/// # Summary
	///
	/// The ident used to refer to the rows matching some [`MatchLocation`] at the end of a `WITH
	/// RECURSIVE`.
	pub(crate) const fn report() -> Self
	{
		Self {
			inner: None,
			current: "location_report",
		}
	}
}

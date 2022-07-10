mod display;
mod from;

use core::fmt::Display;

/// Wraps [`Display`] impls to produce a new value that [`Display`]s like a `snake_case`
/// identifier.
///
/// Created to avoid allocating a new [`String`] via `format!` every time this pattern was
/// required, even if it was only needed to append onto another [`String`].
///
/// # Warnings
///
/// * Does not alter case of input.
///
/// # See also
///
/// * [`SnakeCase::push`]
///
/// # Example
///
/// Pretend we're creating aliases to push to a query [`String`].
///
/// ```rust
/// use clinvoice_adapter::fmt::SnakeCase;
/// # use pretty_assertions::assert_eq;
///
/// /* Scenario 1: Eager, very bad */
/// let job_alias = 'J';
/// let job_client_alias = format!("{job_alias}_O"); // allocation
/// let job_client_location_alias = format!("{job_client_alias}_L"); // allocation
///
/// // That's two allocations needed just to create aliases we can refer back to,
/// // which are just getting pushed to an already allocated String.
///
/// /* Scenario 2: Lazy, very good */
/// let job_alias_2 = SnakeCase::from('J');
/// let job_client_alias_2 = job_alias_2.push('O');
/// let job_client_location_alias_2 = job_client_alias_2.push('L');
///
/// // No allocations up until this point
/// assert_eq!(job_alias.to_string(), job_alias_2.to_string());
/// assert_eq!(job_client_alias, job_client_alias_2.to_string());
/// assert_eq!(
///   job_client_location_alias,
///   job_client_location_alias_2.to_string()
/// );
/// ```
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SnakeCase<TLeft, TRight>
where
	TLeft: Display,
	TRight: Display,
{
	/// A [`SnakeCase`] containing multiple words separated by underscores.
	Body(TLeft, TRight),

	/// A [`SnakeCase`] containing no underscores (i.e. only one word).
	Head(TLeft),
}

impl<TLeft, TRight> SnakeCase<TLeft, TRight>
where
	TLeft: Display,
	TRight: Display,
{
	/// Append a new token to the [`SnakeCase`] setting it as the [`TRight`] of a [`SnakeCase::Body`].
	///
	/// # Example
	///
	/// ```rust
	/// use clinvoice_adapter::fmt::SnakeCase;
	/// # use pretty_assertions::assert_eq;
	///
	/// assert_eq!(SnakeCase::from("foo").push("bar").to_string(), "foo_bar");
	/// ```
	pub const fn push<T>(self, token: T) -> SnakeCase<Self, T>
	where
		T: Display,
	{
		SnakeCase::Body(self, token)
	}

	/// Return both sides of the [`SnakeCase::Body`], or [`None`] if this is the [`SnakeCase::Head`].
	///
	/// # Example
	///
	/// ```rust
	/// use clinvoice_adapter::fmt::SnakeCase;
	/// # use pretty_assertions::assert_eq;
	///
	/// let foo = SnakeCase::from("foo");
	/// assert_eq!(foo.slice_end(), None::<(_, _)>);
	///
	/// let foo_bar = foo.push("bar");
	/// if let Some((foo_bar_left, foo_bar_right)) = foo_bar.slice_end()
	/// {
	///   assert_eq!(foo_bar_left.to_string(), "foo");
	///   assert_eq!(*foo_bar_right, "bar");
	/// }
	///
	/// if let Some((foo_bar_asdf_left, foo_bar_asdf_right)) = foo_bar.push("asdf").slice_end()
	/// {
	///   assert_eq!(foo_bar_asdf_left.to_string(), "foo_bar");
	///   assert_eq!(*foo_bar_asdf_right, "asdf");
	/// }
	/// ```
	pub const fn slice_end(&self) -> Option<(&TLeft, &TRight)>
	{
		match self
		{
			SnakeCase::Body(left, right) => Some((left, right)),
			_ => None,
		}
	}
}

use super::id::Id;

pub mod display;

/// # Summary
///
/// A physical space where other `Location`s or
/// [`Organization`](super::organization::Organization)s exist.
pub struct Location<'name>
{
	/// # Summary
	///
	/// The reference number of the [`Location`].
	_id: Id,

	/// # Summary
	///
	/// The reference number of the [`Location`] in which _this_ [`Location`] resides.
	_outer_id: Option<Id>,

	/// # Summary
	///
	/// The name of the [`Location`].
	pub name: &'name str,
}

impl Location<'_>
{
	/// # Summary
	///
	/// Create a new [`Location`] with a generated ID.
	///
	/// # Parameters
	///
	/// * `name`, the name of the location.
	///
	/// # Returns
	///
	/// ```rust
	/// Location { name, _id: /* generated */ };
	/// ```
	pub fn new(name: &str) -> Self
	{
		todo!();
	}

	/// # Summary
	///
	/// Create a new [`Location`] which is inside of `self`.
	///
	/// # Parameters
	///
	/// * `name`, the name of the inner location.
	///
	/// # Returns
	///
	/// ```rust
	/// Location { name, _id: /* generated */, _outside_id: self._id };
	/// ```
	pub fn new_inner(&self, name: &'_ str) -> Location<'_>
	{
		todo!()
	}
}

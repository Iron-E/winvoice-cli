use crate::Wrapper;

use clinvoice_data::Location;

use core::fmt::Display;

pub trait CrudLocation<'name> : Display + Wrapper<Location<'name>>
{
	/// # Summary
	///
	/// Create a new `Location` with a generated ID.
	///
	/// # Parameters
	///
	/// * `name`, the name of the location.
	///
	/// # Returns
	///
	/// ```ignore
	/// Location { name, id: /* generated */ };
	/// ```
	fn create(name: &'_ str) -> Self;

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
	/// ```ignore
	/// Location { name, id: /* generated */, outside_id: self.unroll().id };
	/// ```
	fn create_inner(&self, name: &'_ str) -> Self;
}

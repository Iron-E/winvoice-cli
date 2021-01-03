use crate::Wrapper;

use clinvoice_data::Location;

pub trait CrudLocation<'name, W> where W : Wrapper<Location<'name>>
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
	fn insert(name: &'_ str) -> W;

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
	fn insert_inner(&self, name: &'_ str) -> W;
}

use clinvoice_adapter::data::InsertableLocation;
use super::TomlLocation;

impl<'name> InsertableLocation<'name, TomlLocation<'name>> for TomlLocation<'name>
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
	/// ```ignore
	/// Location { name, id: /* generated */ };
	/// ```
	fn insert(name: &'_ str) -> Self
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
	/// ```ignore
	/// Location { name, id: /* generated */, outside_id: self.0.id };
	/// ```
	fn insert_inner(&self, name: &'_ str) -> Self
	{
		todo!()
	}
}

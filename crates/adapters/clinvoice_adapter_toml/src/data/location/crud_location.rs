use super::TomlLocation;
use clinvoice_adapter::data::{AnyValue, CrudLocation};
use clinvoice_data::Id;
use std::error::Error;

impl<'err, 'name> CrudLocation<'err, 'name> for TomlLocation<'name>
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
	/// Location {name, id: /* generated */};
	/// ```
	fn create(name: &'_ str) -> Result<Self, &'err dyn Error>
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
	/// Location {name, id: /* generated */, outside_id: self.0.id};
	/// ```
	fn create_inner(&self, name: &'_ str) -> Result<Self, &'err dyn Error>
	{
		todo!()
	}

	fn retrieve<'arr>(id: AnyValue<Id>, name: AnyValue<&'_ str>) -> Result<&'arr [Self], &'err dyn Error>
	{
		todo!()
	}
}

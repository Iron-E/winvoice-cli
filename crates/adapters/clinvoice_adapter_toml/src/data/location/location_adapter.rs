use super::TomlLocation;
use clinvoice_adapter::{data::{AnyValue, LocationAdapter}, Store};
use clinvoice_data::Id;
use std::error::Error;

impl<'name, 'pass, 'path, 'user> LocationAdapter<'name, 'pass, 'path, 'user>
for TomlLocation<'name, 'pass, 'path, 'user>
{
	/// # Summary
	///
	/// Create a new `Location` with a generated ID.
	///
	/// # Parameters
	///
	/// See [`Location`].
	///
	/// # Returns
	///
	/// ```ignore
	/// Location {name, id: /* generated */};
	/// ```
	fn create(name: &'_ str, store: Store<'pass, 'path, 'user>) -> Result<Self, Box<dyn Error>>
	{
		todo!()
	}

	/// # Summary
	///
	/// Create a new [`Location`] which is inside of `self`.
	///
	/// # Parameters
	///
	/// See [`Location`].
	///
	/// # Returns
	///
	/// ```ignore
	/// Location {name, id: /* generated */, outside_id: self.unroll().id};
	/// ```
	fn create_inner(&self, name: &'_ str) -> Result<Self, Box<dyn Error>>
	{
		todo!()
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: Store<'pass, 'path, 'user>) -> Result<(), Box<dyn Error>>
	{
		todo!()
	}

	/// # Summary
	///
	/// Retrieve a [`Location`] from an active [`Store`](core::Store).
	///
	/// # Parameters
	///
	/// See [`Location`].
	///
	/// # Returns
	///
	/// * An [`Error`], when something goes wrong.
	/// * A list of matches, if there are any.
	fn retrieve<'arr>(
		id: AnyValue<Id>,
		name: AnyValue<&'_ str>,
		store: Store<'pass, 'path, 'user>,
	) -> Result<&'arr [Self], Box<dyn Error>>
	{
		todo!()
	}
}


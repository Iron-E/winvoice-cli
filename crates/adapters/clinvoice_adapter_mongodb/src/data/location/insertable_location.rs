use clinvoice_adapter::insertable::InsertableLocation;
use super::{Location, MongoLocation};

impl<'name> InsertableLocation<Location<'name>, MongoLocation<'name>> for MongoLocation<'name>
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
	/// Location { name, id: /* generated */ };
	/// ```
	fn insert(name: &'_ str) -> MongoLocation<'name>
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
	/// Location { name, id: /* generated */, outside_id: self.0.id };
	/// ```
	fn insert_inner(&self, name: &'_ str) -> MongoLocation<'name>
	{
		todo!()
	}
}


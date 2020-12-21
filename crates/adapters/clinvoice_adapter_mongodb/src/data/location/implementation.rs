use super::MongoLocation;

impl MongoLocation<'_>
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
	/// Location { name, pub id: /* generated */ };
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
	/// Location { name, pub id: /* generated */, pub outside_id: self.pub id };
	/// ```
	pub fn new_inner(&self, name: &'_ str) -> MongoLocation<'_>
	{
		todo!()
	}
}


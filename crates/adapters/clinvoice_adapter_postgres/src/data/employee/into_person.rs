use crate::data::{MongoEmployee, MongoPerson};

impl<'name> Into<MongoPerson<'name>> for MongoEmployee
{
	/// # Summary
	///
	/// Convert the [`MongoEmployee`] to a [`MongoPerson`].
	///
	/// # Returns
	///
	/// The [`MongoPerson`] with `self.0.person_id`.
	fn into(self) -> MongoPerson<'name>
	{
		// SELECT P
		// FROM Person P
		// JOIN Employee E ON E._person_id = P._id;

		todo!();
	}
}

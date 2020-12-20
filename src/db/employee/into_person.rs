use crate::db::person::Person;
use super::Employee;

impl<'name> Into<Person<'name>> for &Employee
{
	/// # Summary
	///
	/// Convert the [`Employee`] to a [`Person`].
	///
	/// # Returns
	///
	/// The [`Person`] with `self._person_id`.
	fn into(self) -> Person<'name>
	{
		// SELECT P
		// FROM Person P
		// JOIN Employee E ON E._person_id = P._id;

		todo!();
	}
}

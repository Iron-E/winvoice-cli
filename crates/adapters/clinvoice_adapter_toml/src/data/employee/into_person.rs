use crate::data::{TomlEmployee, TomlPerson};

impl<'name> Into<TomlPerson<'name>> for &TomlEmployee
{
	/// # Summary
	///
	/// Convert the [`TomlEmployee`] to a [`TomlPerson`].
	///
	/// # Returns
	///
	/// The [`TomlPerson`] with `self.0.person_id`.
	fn into(self) -> TomlPerson<'name>
	{
		// SELECT P
		// FROM Person P
		// JOIN Employee E ON E._person_id = P._id;

		todo!();
	}
}

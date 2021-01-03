use crate::data::{TomlEmployee, TomlPerson};

impl<'addr, 'contact_info, 'email, 'name> Into<TomlPerson<'addr, 'contact_info, 'email, 'name>> for &TomlEmployee<'addr, 'contact_info, 'email>
{
	/// # Summary
	///
	/// Convert the [`TomlEmployee`] to a [`TomlPerson`].
	///
	/// # Returns
	///
	/// The [`TomlPerson`] with `self.0.person_id`.
	fn into(self) -> TomlPerson<'addr, 'contact_info, 'email, 'name>
	{
		// SELECT P
		// FROM Person P
		// JOIN Employee E ON E._person_id = P._id;

		todo!();
	}
}

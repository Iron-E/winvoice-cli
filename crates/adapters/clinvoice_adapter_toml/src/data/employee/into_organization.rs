use crate::data::{TomlEmployee, TomlOrganization};

impl<'name> Into<TomlOrganization<'name>> for &TomlEmployee<'_, '_, '_, '_>
{
	/// # Summary
	///
	/// Convert the [`TomlEmployee`] to an [`TomlEmployer`].
	///
	/// # Returns
	///
	/// The [`TomlEmployer`] with `self.0.employer_id`.
	fn into(self) -> TomlOrganization<'name>
	{
		// SELECT Er
		// FROM Employer Er
		// JOIN Employee Ee ON Ee.employer_id = Em.id;

		todo!();
	}
}

use crate::data::{TomlEmployee, TomlEmployer};

impl Into<TomlEmployer> for &TomlEmployee
{
	/// # Summary
	///
	/// Convert the [`TomlEmployee`] to an [`TomlEmployer`].
	///
	/// # Returns
	///
	/// The [`TomlEmployer`] with `self.0.employer_id`.
	fn into(self) -> TomlEmployer
	{
		// SELECT Er
		// FROM Employer Er
		// JOIN Employee Ee ON Ee.employer_id = Em.id;

		todo!();
	}
}

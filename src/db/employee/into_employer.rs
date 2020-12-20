use crate::db::employer::Employer;
use super::Employee;

impl Into<Employer> for &Employee
{
	/// # Summary
	///
	/// Convert the [`Employee`] to an [`Employer`].
	///
	/// # Returns
	///
	/// The [`Employer`] with `self._employer_id`.
	fn into(self) -> Employer
	{
		// SELECT Er
		// FROM Employer Er
		// JOIN Employee Ee ON Ee._employer_id = Em._ed;

		todo!();
	}
}

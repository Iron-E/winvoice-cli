use crate::data::{employee::MongoEmployee, employer::MongoEmployer};

impl Into<MongoEmployer> for &MongoEmployee
{
	/// # Summary
	///
	/// Convert the [`MongoEmployee`] to an [`MongoEmployer`].
	///
	/// # Returns
	///
	/// The [`MongoEmployer`] with `self.0.employer_id`.
	fn into(self) -> MongoEmployer
	{
		// SELECT Er
		// FROM Employer Er
		// JOIN Employee Ee ON Ee.employer_id = Em.id;

		todo!();
	}
}

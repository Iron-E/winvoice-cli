use crate::db::employer::Employer;
use super::Employee;

impl Into<Employer> for &Employee
{
	/// TODO
	fn into(self) -> Employer
	{
		// TODO
		//
		//	SELECT Er
		//	FROM Employer Er
		//	JOIN Employee Ee ON Ee._employer_id = Em._ed;

		todo!();
	}
}

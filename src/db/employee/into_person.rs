use crate::db::person::Person;
use super::Employee;

impl<'name> Into<Person<'name>> for &Employee
{
	/// TODO
	fn into(self) -> Person<'name>
	{
		// TODO
		//
		//	SELECT P
		//	FROM Person P
		//	JOIN Employee E ON E._person_id = P._id;

		todo!();
	}
}

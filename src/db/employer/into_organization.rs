use crate::db::organization::Organization;
use super::Employer;

impl<'name> Into<Organization<'name>> for &Employer
{
	/// TODO
	fn into(self) -> Organization<'name>
	{
		// TODO
		//
		//	SELECT O
		//	FROM Employer E
		//	JOIN Organization O ON E._organization_id = O._id;

		todo!();
	}
}


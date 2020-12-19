use crate::db::organization::Organization;
use super::Client;

impl<'name> Into<Organization<'name>> for &Client
{
	/// TODO
	fn into(self) -> Organization<'name>
	{
		// TODO
		//
		//	SELECT O
		//	FROM Client C
		//	JOIN Organization O ON C._organization_id = O._id;

		todo!();
	}
}

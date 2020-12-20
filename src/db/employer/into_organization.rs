use crate::db::organization::Organization;
use super::Employer;

impl<'name> Into<Organization<'name>> for &Employer
{
	/// # Summary
	///
	/// Convert the [`Employer`] to an [`Organization`].
	///
	/// # Returns
	///
	/// The [`Organization`] with `self._organization_id`.
	fn into(self) -> Organization<'name>
	{
		// SELECT O
		// FROM Employer E
		// JOIN Organization O ON E._organization_id = O._id;

		todo!();
	}
}


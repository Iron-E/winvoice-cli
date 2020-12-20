use crate::db::organization::Organization;
use super::Client;

impl<'name> Into<Organization<'name>> for &Client
{
	/// # Summary
	///
	/// Convert the [`Client`] to an [`Organization`].
	///
	/// # Returns
	///
	/// The [`Organization`] with `self._organization_id`.
	fn into(self) -> Organization<'name>
	{
		// SELECT O
		// FROM Client C
		// JOIN Organization O ON C._organization_id = O._id;

		todo!();
	}
}

use crate::data::{MongoEmployer, MongoOrganization};

impl<'name> Into<MongoOrganization<'name>> for MongoEmployer
{
	/// # Summary
	///
	/// Convert the [`MongoEmployer`] to an [`MongoOrganization`].
	///
	/// # Returns
	///
	/// The [`MongoOrganization`] with `self.0.organization_id`.
	fn into(self) -> MongoOrganization<'name>
	{
		// SELECT O
		// FROM Employer E
		// JOIN Organization O ON E.organization_id = O.id;

		todo!();
	}
}


use crate::data::{client::MongoClient, organization::MongoOrganization};

impl<'name> Into<MongoOrganization<'name>> for &MongoClient
{
	/// # Summary
	///
	/// Convert the [`MongoClient`] to an [`MongoOrganization`].
	///
	/// # Returns
	///
	/// The [`MongoOrganization`] with `self.0.organization_id`.
	fn into(self) -> MongoOrganization<'name>
	{
		// SELECT O
		// FROM Client C
		// JOIN Organization O ON C.organization_id = O.id;

		todo!();
	}
}

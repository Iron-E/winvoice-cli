use crate::data::{location::MongoLocation, organization::MongoOrganization};

impl<'location_name, 'org_name> Into<MongoLocation<'location_name>> for &MongoOrganization<'org_name>
{
	/// # Summary
	///
	/// Convert the [`Organization`] to a [`Location`].
	///
	/// # Returns
	///
	/// The [`Location`] with `self.location_id`.
	fn into(self) -> MongoLocation<'location_name>
	{
		// SELECT L
		// FROM Organization O
		// JOIN Location L ON O.location_id = L.id;

		todo!();
	}
}

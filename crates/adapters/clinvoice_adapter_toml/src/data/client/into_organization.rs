use crate::data::{TomlClient, TomlOrganization};

impl<'name> Into<TomlOrganization<'name>> for &TomlClient
{
	/// # Summary
	///
	/// Convert the [`TomlClient`] to an [`TomlOrganization`].
	///
	/// # Returns
	///
	/// The [`TomlOrganization`] with `self.0.organization_id`.
	fn into(self) -> TomlOrganization<'name>
	{
		// SELECT O
		// FROM Client C
		// JOIN Organization O ON C.organization_id = O.id;

		todo!();
	}
}

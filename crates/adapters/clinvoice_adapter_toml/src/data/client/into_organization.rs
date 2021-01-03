use crate::data::{TomlClient, TomlOrganization};

impl<'name, 'rep_title> Into<TomlOrganization<'name, 'rep_title>> for &TomlClient
{
	/// # Summary
	///
	/// Convert the [`TomlClient`] to an [`TomlOrganization`].
	///
	/// # Returns
	///
	/// The [`TomlOrganization`] with `self.0.organization_id`.
	fn into(self) -> TomlOrganization<'name, 'rep_title>
	{
		// SELECT O
		// FROM Client C
		// JOIN Organization O ON C.organization_id = O.id;

		todo!();
	}
}

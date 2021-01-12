use crate::data::{location::TomlLocation, organization::TomlOrganization};

impl<'loc_name, 'org_name> Into<TomlLocation<'loc_name>> for &TomlOrganization<'org_name>
{
	/// # Summary
	///
	/// Convert the [`Organization`] to a [`Location`].
	///
	/// # Returns
	///
	/// The [`Location`] with `self.location_id`.
	fn into(self) -> TomlLocation<'loc_name>
	{
		// SELECT L
		// FROM Organization O
		// JOIN Location L ON O.location_id = L.id;

		todo!();
	}
}

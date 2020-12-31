use crate::data::{location::TomlLocation, organization::TomlOrganization};

impl<'location_name, 'org_name> Into<TomlLocation<'location_name>> for &TomlOrganization<'org_name>
{
	/// # Summary
	///
	/// Convert the [`Organization`] to a [`Location`].
	///
	/// # Returns
	///
	/// The [`Location`] with `self.location_id`.
	fn into(self) -> TomlLocation<'location_name>
	{
		// SELECT L
		// FROM Organization O
		// JOIN Location L ON O.location_id = L.id;

		todo!();
	}
}

use crate::data::{TomlEmployer, TomlOrganization};

impl<'name> Into<TomlOrganization<'name>> for &TomlEmployer
{
	/// # Summary
	///
	/// Convert the [`TomlEmployer`] to an [`TomlOrganization`].
	///
	/// # Returns
	///
	/// The [`TomlOrganization`] with `self.0.organization_id`.
	fn into(self) -> TomlOrganization<'name>
	{
		// SELECT O
		// FROM Employer E
		// JOIN Organization O ON E.organization_id = O.id;

		todo!();
	}
}


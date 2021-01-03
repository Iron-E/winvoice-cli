use crate::data::{TomlEmployer, TomlOrganization};

impl<'name, 'rep_title> Into<TomlOrganization<'name, 'rep_title>> for &TomlEmployer
{
	/// # Summary
	///
	/// Convert the [`TomlEmployer`] to an [`TomlOrganization`].
	///
	/// # Returns
	///
	/// The [`TomlOrganization`] with `self.0.organization_id`.
	fn into(self) -> TomlOrganization<'name, 'rep_title>
	{
		// SELECT O
		// FROM Employer E
		// JOIN Organization O ON E.organization_id = O.id;

		todo!();
	}
}


mod deletable;
mod into_hashset_employee_result;
mod into_location_result;
mod organization_adapter;
mod updatable;

use std::path::PathBuf;

clinvoice_adapter::Adapt!(Organization => TomlOrganization);

impl<'path> TomlOrganization<'_, '_, 'path, '_>
{
	/// # Summary
	///
	/// Return the directory within `store` that contains information about [`TomlEmployee`]s.
	///
	/// # Parameters
	///
	/// * `store`, the [`Store`] whose `path` should be used to reference information about
	///   [`TomlEmployee`]s.
	///
	/// # Returns
	///
	/// The [`Path`] leading to where [`TomlEmployee`]s are in `store`.
	pub fn path(store: &Store<'_, 'path, '_>) -> PathBuf
	{
		return PathBuf::new().join(store.path).join("Organizations");
	}

	/// # Summary
	///
	/// Get the [`PathBuf`] pointing to where this [`TomlOrganization`] is stored.
	///
	/// # Returns
	///
	/// A [`PathBuf`] pointing to where this [`TomlOrganization`] is stored.
	pub fn filepath(&self) -> PathBuf
	{
		return TomlOrganization::path(&self.store).join(self.organization.id.to_string());
	}
}

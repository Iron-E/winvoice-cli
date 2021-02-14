mod deletable;
mod initializable;
mod into_hashset_employee_result;
mod into_location_result;
mod organization_adapter;
mod updatable;

use std::path::PathBuf;

clinvoice_adapter::Adapt!(Organization => BincodeOrganization);

impl<'path> BincodeOrganization<'_, 'path, '_>
{
	/// # Summary
	///
	/// Return the directory within `store` that contains information about [`BincodeEmployee`]s.
	///
	/// # Parameters
	///
	/// * `store`, the [`Store`] whose `path` should be used to reference information about
	///   [`BincodeEmployee`]s.
	///
	/// # Returns
	///
	/// The [`Path`] leading to where [`BincodeEmployee`]s are in `store`.
	pub fn path(store: &Store<'_, 'path, '_>) -> PathBuf
	{
		return PathBuf::new().join(store.path).join("Organizations");
	}

	/// # Summary
	///
	/// Get the [`PathBuf`] pointing to where this [`BincodeOrganization`] is stored.
	///
	/// # Returns
	///
	/// A [`PathBuf`] pointing to where this [`BincodeOrganization`] is stored.
	pub fn filepath(&self) -> PathBuf
	{
		return Self::path(&self.store).join(self.organization.id.to_string());
	}
}

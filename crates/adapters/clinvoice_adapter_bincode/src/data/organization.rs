mod deletable;
mod initializable;
mod organization_adapter;
mod updatable;

use std::path::PathBuf;

use crate::util;

clinvoice_adapter::Adapt!(Organization => BincodeOrganization);

impl BincodeOrganization<'_, '_>
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
	pub fn path(store: &Store) -> PathBuf
	{
		util::expand_store_path(store).join("Organizations")
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
		Self::path(&self.store).join(self.organization.id.to_string())
	}
}

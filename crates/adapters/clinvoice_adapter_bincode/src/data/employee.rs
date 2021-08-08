mod deletable;
mod employee_adapter;
mod initializable;
mod updatable;

use std::path::PathBuf;

use crate::util;

clinvoice_adapter::Adapt!(Employee => BincodeEmployee);

impl BincodeEmployee<'_, '_>
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
		util::expand_store_path(store).join("Employees")
	}

	/// # Summary
	///
	/// Get the [`PathBuf`] pointing to where this [`BincodeEmployee`] is stored.
	///
	/// # Returns
	///
	/// A [`PathBuf`] pointing to where this [`BincodeEmployee`] is stored.
	pub fn filepath(&self) -> PathBuf
	{
		Self::path(self.store).join(self.employee.id.to_string())
	}
}

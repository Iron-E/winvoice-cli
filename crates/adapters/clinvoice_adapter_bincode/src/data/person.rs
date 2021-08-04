mod deletable;
mod initializable;
mod person_adapter;
mod updatable;

use std::path::PathBuf;

use crate::util;

clinvoice_adapter::Adapt!(Person => BincodePerson);

impl BincodePerson<'_, '_>
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
		util::expand_store_path(store).join("People")
	}

	/// # Summary
	///
	/// Get the [`PathBuf`] pointing to where this [`BincodePerson`] is stored.
	///
	/// # Returns
	///
	/// A [`PathBuf`] pointing to where this [`BincodePerson`] is stored.
	pub fn filepath(&self) -> PathBuf
	{
		Self::path(&self.store).join(self.person.id.to_string())
	}
}

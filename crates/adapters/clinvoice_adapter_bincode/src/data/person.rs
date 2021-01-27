mod deletable;
mod person_adapter;
mod updatable;

use std::path::PathBuf;

clinvoice_adapter::Adapt!(Person => BincodePerson);

impl<'path> BincodePerson<'_, '_, '_, '_, 'path, '_>
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
		return PathBuf::new().join(store.path).join("People");
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
		return BincodePerson::path(&self.store).join(self.person.id.to_string());
	}
}

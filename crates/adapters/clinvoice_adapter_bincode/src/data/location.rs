mod deletable;
mod display;
mod location_adapter;
mod updatable;

use std::path::PathBuf;

clinvoice_adapter::Adapt!(Location => BincodeLocation);

impl<'path> BincodeLocation<'_, '_, 'path, '_>
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
		return PathBuf::new().join(store.path).join("Locations");
	}

	/// # Summary
	///
	/// Get the [`PathBuf`] pointing to where this [`BincodeLocation`] is stored.
	///
	/// # Returns
	///
	/// A [`PathBuf`] pointing to where this [`BincodeLocation`] is stored.
	pub fn filepath(&self) -> PathBuf
	{
		return Self::path(&self.store).join(self.location.id.to_string());
	}
}

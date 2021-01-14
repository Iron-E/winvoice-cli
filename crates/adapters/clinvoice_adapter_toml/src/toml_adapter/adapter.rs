use super::TomlAdapter;
use crate::paths;
use clinvoice_adapter::{Adapter, AdapterMismatchError, Adapters, Store};
use std::{fs, io::{Error as IOError, ErrorKind}, path::Path};

impl<'path, 'pass, 'user> Adapter<'path, 'pass, 'user, IOError> for TomlAdapter<'path, 'pass, 'user>
{
	/// # Summary
	///
	/// Retrieve the current [`Store`].
	fn active_store(&self) -> &Store<'path, 'pass, 'user>
	{
		return &self.store;
	}

	/// # Summary
	///
	/// Initialize the postgresql database on [`Store`].
	fn init(&self) -> Result<(), IOError>
	{
		let store_path = Path::new(self.store.path);

		if store_path.exists() && store_path.read_dir()?.count() > 0
		{
			return Err(IOError::new(
				ErrorKind::AlreadyExists, format!("The specified path, {}, is already in use.", self.store.path)
			));
		}

		fs::create_dir_all(store_path)?;
		for dir in vec![paths::EMPLOYEE, paths::JOB, paths::LOCATION, paths::ORGANIZATION, paths::PERSON]
		{
			fs::create_dir(store_path.join(Path::new(dir)))?;
		}

		return Ok(());
	}

	/// # Summary
	///
	/// Create a new [`TomlAdapter`].
	///
	/// # Paramters
	///
	/// * `store`, the [`Store`] to manage.
	///
	/// # Returns
	///
	/// * A new [`TomlAdapter`], if `store.adapter` is [`TOML`](crate::Adapters::TOML).
	/// * `AdapterMismatchError` if `store.adapter` is not [`TOML`](crate::Adapters::TOML).
	fn new<'msg>(store: Store<'path, 'pass, 'user>) -> Result<Self, AdapterMismatchError<'msg>>
	{
		return match store.adapter
		{
			Adapters::TOML => Ok(TomlAdapter {store}),
			_ => Err(Adapters::TOML.mismatch(store.adapter)),
		}
	}
}

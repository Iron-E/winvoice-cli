use super::TomlAdapter;

use clinvoice_adapter::{Adapter, Store};

use std::io::Error as IOError;

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
		todo!()
	}

	/// # Summary
	///
	/// Create a new [`TomlAdapter`].
	///
	/// # Remarks
	///
	/// # Panics
	///
	/// If `store.adapter` is not [`toml`](crate::Adapters::toml).
	fn new(store: Store<'path, 'pass, 'user>) -> Self
	{
		return TomlAdapter { store };
	}
}

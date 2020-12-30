use clinvoice_adapter::{Adapter, Store};
use super::PostgresAdapter;

use postgres::Error as PostgresError;

impl<'path, 'pass, 'user> Adapter<'path, 'pass, 'user, PostgresError> for PostgresAdapter<'path, 'pass, 'user>
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
	fn init() -> Result<(), PostgresError>
	{
		todo!()
	}

	/// # Summary
	///
	/// Create a new [`PostgresAdapter`].
	///
	/// # Remarks
	///
	/// # Panics
	///
	/// If `store.adapter` is not [`POSTGRES`](crate::Adapters::POSTGRES).
	fn new(store: Store<'path, 'pass, 'user>) -> Self
	{
		return PostgresAdapter { store };
	}
}

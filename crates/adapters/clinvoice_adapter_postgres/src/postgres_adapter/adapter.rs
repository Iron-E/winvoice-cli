use clinvoice_adapter::{Adapter, Connection};
use super::PostgresAdapter;

use postgres::Error as PostgresError;

impl<'db, 'url> Adapter<'db, 'url, PostgresError> for PostgresAdapter<'db, 'url>
{
	/// # Summary
	///
	/// Retrieve the current [`Connection`].
	fn current_connection(self) -> Connection<'db, 'url>
	{
		return self.connection;
	}

	/// # Summary
	///
	/// Initialize the postgresql database on [`Connection`].
	fn init() -> Result<(), PostgresError>
	{
		todo!()
	}

	fn new(connection: Connection<'db, 'url>) -> Self
	{
		return PostgresAdapter { connection };
	}
}

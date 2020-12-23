mod adapter;

use clinvoice_adapter::Connection;

pub struct PostgresAdapter<'db, 'url>
{
	connection: Connection<'db, 'url>
}

mod adapter;

use clinvoice_adapter::Store;

pub struct PostgresAdapter<'path, 'pass, 'user>
{
	store: Store<'path, 'pass, 'user>,
}

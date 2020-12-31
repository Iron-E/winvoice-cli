mod adapter;

use clinvoice_adapter::Store;

pub struct TomlAdapter<'path, 'pass, 'user>
{
	store: Store<'path, 'pass, 'user>,
}

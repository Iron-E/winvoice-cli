use clinvoice_adapter::Store;

pub enum StoreValue<'alias, 'db, 'pass, 'path, 'user>
{
	Alias(&'alias str),
	Storage(Store<'db, 'pass, 'path, 'user>),
}

use crate::Adapters;

pub enum Store<'db, 'pass, 'path, 'user>
{
	Database
	{
		adapter: Adapters,
		database: &'db str,
		password: &'pass str,
		username: &'user str,
	},

	FileSystem
	{
		adapter: Adapters,
		root: &'path str,
	},
}

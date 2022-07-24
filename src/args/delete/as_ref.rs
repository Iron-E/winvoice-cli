use super::Delete;
use crate::args::store_args::StoreArgs;

impl AsRef<StoreArgs> for Delete
{
	fn as_ref(&self) -> &StoreArgs
	{
		&self.store_args
	}
}

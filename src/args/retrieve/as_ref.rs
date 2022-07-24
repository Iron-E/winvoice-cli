use super::Retrieve;
use crate::args::store_args::StoreArgs;

impl AsRef<StoreArgs> for Retrieve
{
	fn as_ref(&self) -> &StoreArgs
	{
		&self.store_args
	}
}

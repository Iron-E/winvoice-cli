use super::Create;
use crate::args::store_args::StoreArgs;

impl AsRef<StoreArgs> for Create
{
	fn as_ref(&self) -> &StoreArgs
	{
		&self.store_args
	}
}

use super::Update;
use crate::args::store_args::StoreArgs;

impl AsRef<StoreArgs> for Update
{
	fn as_ref(&self) -> &StoreArgs
	{
		&self.store_args
	}
}

use clinvoice_adapter::Wrapper;
use super::{Location, MongoLocation};

impl<'name> Wrapper<Location<'name>> for MongoLocation<'name>
{
	/// # Summary
	///
	/// Get the inner [`Location`].
	fn unroll(self) -> Location<'name>
	{
		return self.0;
	}
}

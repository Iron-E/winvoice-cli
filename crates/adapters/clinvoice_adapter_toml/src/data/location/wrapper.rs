use clinvoice_adapter::Wrapper;
use super::{Location, TomlLocation};

impl<'name> Wrapper<Location<'name>> for TomlLocation<'name>
{
	/// # Summary
	///
	/// Get the inner [`Location`].
	fn unroll(self) -> Location<'name>
	{
		return self.0;
	}
}

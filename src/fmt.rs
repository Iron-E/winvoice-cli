//! Tools to format data.

use core::any;

/// The [`type_name`](any::type_name) without any leading module names.
pub fn type_name<T>() -> &'static str
{
	any::type_name::<T>()
		.split("::")
		.last()
		.expect("`T` should have a type name")
}

#[cfg(test)]
mod tests
{
	use clinvoice_adapter::schema::columns::JobColumns;

	use crate::fmt;

	#[test]
	fn type_name()
	{
		assert_eq!("JobColumns<&str>", fmt::type_name::<JobColumns<&str>>());
	}
}

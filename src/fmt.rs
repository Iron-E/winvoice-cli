//! Tools to format data.

use core::{any, fmt::Display};

use clinvoice_schema::Id;

/// Return "№{id}" (without quotes).
pub(crate) fn id_num(id: Id) -> String
{
	format!("№{id}")
}

/// Return "{t}" (with quotes).
pub(crate) fn quoted<T>(t: T) -> String
where
	T: Display,
{
	format!(r#""{t}""#)
}

/// The [`type_name`](any::type_name) without any leading module names.
pub(crate) fn type_name<T>() -> &'static str
{
	any::type_name::<T>().split("::").last().expect("`T` should have a type name")
}

#[cfg(test)]
mod tests
{
	use clinvoice_adapter::schema::columns::JobColumns;

	use crate::fmt;

	#[test]
	fn id_num()
	{
		assert_eq!("№3", fmt::id_num(3));
	}

	#[test]
	fn type_name()
	{
		assert_eq!("JobColumns<&str>", fmt::type_name::<JobColumns<&str>>());
	}
}

use
{
	std::{borrow::Cow::Borrowed, fs, io::ErrorKind},

	super::PostgresPerson,
	crate::data::{PostgresEmployee, Error, Result},

	clinvoice_adapter::data::{Deletable, EmployeeAdapter, Error as DataError},
	clinvoice_query as query,
};

impl Deletable for PostgresPerson<'_, '_>
{
	type Error = Error;

	fn delete(&self, cascade: bool) -> Result<()>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	#[test]
	fn delete()
	{
	}
}

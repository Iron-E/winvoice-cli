use
{
	std::{borrow::Cow::Borrowed, fs, io::ErrorKind},

	super::PostgresOrganization,
	crate::data::{PostgresEmployee, PostgresJob, Error, Result},

	clinvoice_adapter::data::{Deletable, EmployeeAdapter, Error as DataError, JobAdapter},
	clinvoice_data::Employee,
	clinvoice_query as query,
};

impl Deletable for PostgresOrganization<'_, '_>
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

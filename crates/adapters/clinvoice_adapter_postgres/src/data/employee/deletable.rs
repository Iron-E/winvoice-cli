use
{
	std::{borrow::Cow::Borrowed, fs, io::ErrorKind},

	super::PostgresEmployee,
	crate::data::{PostgresJob, Error, Result},

	clinvoice_adapter::data::{Deletable, Error as DataError, JobAdapter, Updatable},
	clinvoice_query as query,
};

impl Deletable for PostgresEmployee<'_, '_>
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

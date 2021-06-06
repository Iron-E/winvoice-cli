use
{
	std::{fs, io::ErrorKind},

	super::PostgresJob,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Deletable,
};

impl Deletable for PostgresJob<'_, '_>
{
	type Error = Error;

	fn delete(&self, _cascade: bool) -> Result<()>
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

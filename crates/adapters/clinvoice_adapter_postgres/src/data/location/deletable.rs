use
{
	std::{borrow::Cow::Borrowed, fs, io::ErrorKind},

	super::PostgresLocation,
	crate::data::{PostgresOrganization, Error, Result},

	clinvoice_adapter::data::{Deletable, Error as DataError, LocationAdapter, OrganizationAdapter},
	clinvoice_data::Location,
	clinvoice_query as query,
};

impl Deletable for PostgresLocation<'_, '_>
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

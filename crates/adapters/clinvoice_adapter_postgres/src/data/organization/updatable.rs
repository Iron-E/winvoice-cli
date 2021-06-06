use
{
	std::fs,

	super::PostgresOrganization,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Updatable,
};

impl Updatable for PostgresOrganization<'_, '_>
{
	type Error = Error;

	fn update(&self) -> Result<()>
	{
		todo!()
	}
}

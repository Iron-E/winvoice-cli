use
{
	std::fs,

	super::PostgresJob,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Updatable,
};

impl Updatable for PostgresJob<'_, '_>
{
	type Error = Error;

	fn update(&self) -> Result<()>
	{
		todo!()
	}
}

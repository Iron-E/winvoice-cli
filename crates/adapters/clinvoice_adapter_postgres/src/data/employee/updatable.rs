use
{
	std::fs,

	super::PostgresEmployee,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Updatable,
};

impl Updatable for PostgresEmployee<'_, '_>
{
	type Error = Error;

	fn update(&self) -> Result<()>
	{
		todo!()
	}
}

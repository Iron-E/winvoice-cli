use
{
	std::fs,

	super::PostgresLocation,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Updatable,
};

impl Updatable for PostgresLocation<'_, '_>
{
	type Error = Error;

	fn update(&self) -> Result<()>
	{
		todo!()
	}
}

use
{
	std::fs,

	super::PostgresPerson,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Updatable,
};

impl Updatable for PostgresPerson<'_, '_>
{
	type Error = Error;

	fn update(&self) -> Result<()>
	{
		todo!()
	}
}

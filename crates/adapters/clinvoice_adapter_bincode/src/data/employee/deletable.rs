use
{
	super::BincodeEmployee,
	clinvoice_adapter::data::Deletable,
	std::error::Error,
};

impl Deletable for BincodeEmployee<'_, '_, '_>
{
	fn delete(&self, cascade: bool) -> Result<(), Box<dyn Error>>
	{
		todo!()
	}
}

use
{
	super::BincodeLocation,
	clinvoice_adapter::DynamicResult,
	clinvoice_data::views::LocationView,
};

impl Into<DynamicResult<LocationView>> for BincodeLocation<'_, '_, '_>
{
	fn into(self) -> DynamicResult<LocationView>
	{
		todo!();
	}
}



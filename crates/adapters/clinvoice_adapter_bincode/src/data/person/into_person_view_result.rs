use
{
	super::BincodePerson,
	clinvoice_adapter::DynamicResult,
	clinvoice_data::views::PersonView,
};

impl Into<DynamicResult<PersonView>> for BincodePerson<'_, '_, '_>
{
	fn into(self) -> DynamicResult<PersonView>
	{
		todo!();
	}
}





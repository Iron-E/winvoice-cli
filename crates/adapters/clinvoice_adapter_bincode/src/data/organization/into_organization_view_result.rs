use
{
	super::BincodeOrganization,
	clinvoice_adapter::DynamicResult,
	clinvoice_data::views::OrganizationView,
};

impl Into<DynamicResult<OrganizationView>> for BincodeOrganization<'_, '_, '_>
{
	fn into(self) -> DynamicResult<OrganizationView>
	{
		todo!();
	}
}




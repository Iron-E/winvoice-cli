use
{
	super::BincodeOrganization,
	crate::data::BincodeLocation,
	clinvoice_adapter::DynamicResult,
	clinvoice_data::
	{
		Location,
		views::{LocationView, OrganizationView},
	},
};

impl Into<DynamicResult<OrganizationView>> for BincodeOrganization<'_, '_, '_>
{
	fn into(self) -> DynamicResult<OrganizationView>
	{
		let location_result: DynamicResult<Location> = self.clone().into();
		let location_view_result: DynamicResult<LocationView> = BincodeLocation
		{
			location: location_result?,
			store: self.store,
		}.into();

		return Ok(OrganizationView
		{
			location: location_view_result?,
			name: self.organization.name,
		});
	}
}

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

// NOTE: tests not needed because this is called in `Into<EmployeeView>`
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

		Ok(OrganizationView
		{
			id: self.organization.id,
			location: location_view_result?,
			name: self.organization.name,
		})
	}
}

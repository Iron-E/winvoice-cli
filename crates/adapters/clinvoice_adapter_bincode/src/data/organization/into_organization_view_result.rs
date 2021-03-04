use
{
	super::BincodeOrganization,
	crate::data::{BincodeLocation, Result},
	clinvoice_data::
	{
		Location,
		views::{LocationView, OrganizationView},
	},
};

// NOTE: tests not needed because this is called in `Into<EmployeeView>`
impl Into<Result<OrganizationView>> for BincodeOrganization<'_>
{
	fn into(self) -> Result<OrganizationView>
	{
		let id = self.organization.id;
		let name = self.organization.name.clone();
		let store = self.store;

		let location_result: Result<Location> = self.into();
		let location_view_result: Result<LocationView> = BincodeLocation
		{
			location: location_result?,
			store,
		}.into();

		Ok(OrganizationView
		{
			id,
			location: location_view_result?,
			name,
		})
	}
}

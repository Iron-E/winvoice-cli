use
{
	super::BincodeLocation,
	clinvoice_adapter::{data::LocationAdapter, DynamicResult},
	clinvoice_data::views::LocationView,
};

impl Into<DynamicResult<LocationView>> for BincodeLocation<'_, '_, '_>
{
	fn into(self) -> DynamicResult<LocationView>
	{
		let mut outer_locations = self.outer_locations()?;
		let mut outer_location_views = Vec::<LocationView>::with_capacity(outer_locations.len());

		outer_locations.reverse();

		for i in 0..outer_locations.len()
		{
			outer_location_views.push(LocationView::new(
				outer_locations[i].name.clone(),
				outer_location_views.get(i-1),
			));
		}

		return Ok(LocationView::new(
			self.location.name,
			outer_location_views.last(),
		));
	}
}

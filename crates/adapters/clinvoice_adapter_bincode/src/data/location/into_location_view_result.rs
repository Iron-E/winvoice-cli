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
			let outer_location = &outer_locations[i];
			outer_location_views.push(LocationView::new(
				outer_location.id,
				outer_location.name.clone(),
				match i
				{
					0 => None,
					_ => Some(&outer_location_views[i-1]),
				},
			));
		}

		return Ok(LocationView::new(
			self.location.id,
			self.location.name,
			outer_location_views.last(),
		));
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeLocation, DynamicResult, LocationAdapter, LocationView},
		crate::util,
		std::time::Instant,
	};

	/// # NOTE
	///
	/// Technically this test is not needed, because this function is called as a part of
	/// `Into<EmployeeView>`. However, this is a good example of a complicated case that needed to
	/// be tested.
	#[test]
	fn test_into_view()
	{
		let start = Instant::now();

		util::test_temp_store(|store|
		{
			let earth = BincodeLocation::create("Earth", *store).unwrap();
			let usa = earth.create_inner("USA").unwrap();
			let arizona = usa.create_inner("Arizona").unwrap();
			let phoenix = arizona.create_inner("Phoenix").unwrap();

			let phoenix_view = LocationView
			{
				id: phoenix.location.id,
				name: phoenix.location.name.clone(),
				outer: Some(LocationView
				{
					id: arizona.location.id,
					name: arizona.location.name,
					outer: Some(LocationView
					{
						id: usa.location.id,
						name: usa.location.name,
						outer: Some(LocationView
						{
							id: earth.location.id,
							name: earth.location.name,
							outer: None,
						}.into()),
					}.into()),
				}.into()),
			};

			let phoenix_view_result: DynamicResult<LocationView> = phoenix.into();

			assert_eq!(phoenix_view, phoenix_view_result.unwrap());

			println!("\n>>>>> BincodeLocation::into_view {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}
}

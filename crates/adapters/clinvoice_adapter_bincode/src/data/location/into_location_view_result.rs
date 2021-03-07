use
{
	super::BincodeLocation,
	crate::data::Result,
	clinvoice_adapter::data::LocationAdapter,
	clinvoice_data::views::LocationView,
};

impl Into<Result<LocationView>> for BincodeLocation<'_, '_>
{
	fn into(self) -> Result<LocationView>
	{
		let id = self.location.id;
		let name = self.location.name.clone();

		let mut outer_locations = self.outer_locations()?;
		outer_locations.reverse();

		Ok(LocationView
		{
			id,
			name,
			outer: outer_locations.into_iter().fold(None,
				|previous: Option<LocationView>, outer_location| Some(LocationView
				{
					id: outer_location.id,
					name: outer_location.name,
					outer: previous.map(|l| l.into()),
				}),
			).map(|l| l.into()),
		})
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeLocation, Result, LocationAdapter, LocationView},
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
		util::test_temp_store(|store|
		{
			let earth = BincodeLocation::create("Earth", &store).unwrap();

			let usa = BincodeLocation
			{
				location: &earth,
				store,
			}.create_inner("USA").unwrap();

			let arizona = BincodeLocation
			{
				location: &usa,
				store,
			}.create_inner("Arizona").unwrap();

			let phoenix = BincodeLocation
			{
				location: &BincodeLocation
				{
					location: &arizona,
					store,
				}.create_inner("Phoenix").unwrap(),
				store,
			};

			let phoenix_view = LocationView
			{
				id: phoenix.location.id,
				name: phoenix.location.name.clone(),
				outer: Some(LocationView
				{
					id: arizona.id,
					name: arizona.name,
					outer: Some(LocationView
					{
						id: usa.id,
						name: usa.name,
						outer: Some(LocationView
						{
							id: earth.id,
							name: earth.name,
							outer: None,
						}.into()),
					}.into()),
				}.into()),
			};

			let start = Instant::now();
			let phoenix_view_result: Result<LocationView> = phoenix.into();
			println!("\n>>>>> BincodeLocation::into_view {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

			assert_eq!(phoenix_view, phoenix_view_result.unwrap());
		});
	}
}

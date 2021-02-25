use
{
	crate::data::{BincodeLocation, BincodeOrganization},
	clinvoice_adapter::
	{
		data::{Error as DataError, LocationAdapter, MatchWhen},
		DynamicResult,
	},
	clinvoice_data::Location,
};

impl Into<DynamicResult<Location>> for BincodeOrganization<'_, '_, '_>
{
	fn into(self) -> DynamicResult<Location>
	{
		let results = BincodeLocation::retrieve(
			MatchWhen::EqualTo(self.organization.location_id), // id
			MatchWhen::Any, // name
			MatchWhen::Any, // outer id
			self.store,
		)?;

		let bincode_location = match results.iter().next()
		{
			Some(bin_org) => bin_org,
			_ => return Err(DataError::DataIntegrity {id: self.organization.location_id}.into()),
		};

		return Ok(bincode_location.location.clone());
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeLocation, BincodeOrganization, DynamicResult, LocationAdapter},
		crate::util,
		clinvoice_adapter::data::OrganizationAdapter,
		clinvoice_data::Location,
		std::time::Instant,
	};

	#[test]
	fn test_into_hashset_employee()
	{
		util::test_temp_store(|store|
		{
			let arizona = BincodeLocation::create("Arizona", *store).unwrap();
			let dogood = BincodeOrganization::create(
				arizona.location.clone(),
				"DoGood Inc",
				*store
			).unwrap();

			let start = Instant::now();
			// Retrieve the written employees back into the `Employee` structure.
			let dogood_location: DynamicResult<Location> = dogood.into();
			println!("\n>>>>> BincodeOrganization::into_location {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

			// Assert that the location retrieved is the location expected
			assert_eq!(arizona.location, dogood_location.unwrap());
		});
	}
}

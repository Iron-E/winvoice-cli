use
{
	crate::data::{BincodeLocation, BincodeOrganization, Result},
	clinvoice_adapter::data::{Error as DataError, LocationAdapter, Match, retrieve},
	clinvoice_data::Location,
	std::borrow::Cow,
};

impl Into<Result<Location>> for BincodeOrganization<'_, '_>
{
	fn into(self) -> Result<Location>
	{
		let results = BincodeLocation::retrieve(
			retrieve::Location
			{
				id: Match::EqualTo(Cow::Borrowed(&self.organization.location_id)),
				..Default::default()
			},
			self.store,
		)?;

		let location = match results.get(0)
		{
			Some(loc) => loc,
			_ => return Err(DataError::DataIntegrity {id: self.organization.location_id}.into()),
		};

		Ok(location.clone())
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeLocation, BincodeOrganization, LocationAdapter, Result},
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
			let arizona = BincodeLocation::create("Arizona", &store).unwrap();
			let dogood = BincodeOrganization
			{
				organization: &BincodeOrganization::create(
					arizona.clone(),
					"DoGood Inc",
					&store
				).unwrap(),
				store,
			};

			let start = Instant::now();
			// Retrieve the written employees back into the `Employee` structure.
			let dogood_location: Result<Location> = dogood.into();
			println!("\n>>>>> BincodeOrganization::into_location {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

			// Assert that the location retrieved is the location expected
			assert_eq!(arizona, dogood_location.unwrap());
		});
	}
}

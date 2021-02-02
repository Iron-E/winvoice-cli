use crate::data::{BincodeLocation, BincodeOrganization};
use clinvoice_adapter::data::{Error as DataError, LocationAdapter, MatchWhen};
use clinvoice_data::Location;
use std::error::Error;

impl Into<Result<Location, Box<dyn Error>>> for BincodeOrganization<'_, '_, '_>
{
	fn into(self) -> Result<Location, Box<dyn Error>>
	{
		let results = BincodeLocation::retrieve(
			MatchWhen::EqualTo(self.organization.location_id),
			MatchWhen::Any,
			MatchWhen::Any,
			self.store,
		)?;

		let bincode_location = match results.iter().next()
		{
			Some(bin_org) => bin_org,
			None => Err(DataError::DataIntegrity {id: self.organization.location_id})?,
		};

		return Ok(bincode_location.location.clone());
	}
}

#[cfg(test)]
mod tests
{
	use super::{BincodeLocation, BincodeOrganization, LocationAdapter};
	use crate::util;
	use clinvoice_adapter::data::OrganizationAdapter;
	use clinvoice_data::Location;
	use std::{collections::HashSet, error::Error, time::Instant};

	#[test]
	fn test_into_hashset_employee()
	{
		let start = Instant::now();

		util::test_temp_store(|store|
		{
			let arizona = BincodeLocation::create("Arizona", *store).unwrap();
			let dogood = BincodeOrganization::create(
				arizona.location.clone(),
				"DoGood Inc",
				HashSet::new(),
				*store
			).unwrap();

			// Retrieve the written employees back into the `Employee` structure.
			let dogood_location: Result<Location, Box<dyn Error>> = dogood.into();

			// Assert that the location retrieved is the location expected
			assert_eq!(arizona.location, dogood_location.unwrap());

			println!("\n>>>>> BincodeOrganization test_into_location {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		}).unwrap();
	}
}

use
{
	crate::data::{BincodeJob, BincodeOrganization},
	clinvoice_adapter::data::{Error as DataError, MatchWhen, OrganizationAdapter},
	clinvoice_data::Organization,
	std::error::Error,
};

impl Into<Result<Organization, Box<dyn Error>>> for BincodeJob<'_, '_, '_>
{
	fn into(self) -> Result<Organization, Box<dyn Error>>
	{
		let results = BincodeOrganization::retrieve(
			MatchWhen::EqualTo(self.job.client_id), // id
			MatchWhen::Any, // location
			MatchWhen::Any, // name
			MatchWhen::Any, // representatives
			self.store,
		)?;

		let bincode_organization = match results.iter().next()
		{
			Some(bin_org) => bin_org,
			None => Err(DataError::DataIntegrity {id: self.job.client_id})?,
		};

		return Ok(bincode_organization.organization.clone());
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeJob, BincodeOrganization, OrganizationAdapter},
		crate::util,
		clinvoice_adapter::data::JobAdapter,
		clinvoice_data::{chrono::Utc, Decimal, Id, Location, Money, Organization},
		std::{collections::HashSet, error::Error, time::Instant},
	};

	#[test]
	fn test_into_organization()
	{
		let start = Instant::now();

		util::test_temp_store(|store|
		{
			let dogood = BincodeOrganization::create(
				Location {name: "Earth".into(), id: Id::new_v4(), outer_id: None},
				"DoGood Inc",
				HashSet::new(),
				*store
			).unwrap();

			let test_job = BincodeJob::create(
				dogood.organization.clone(),
				Utc::now(),
				Money::new(Decimal::new(200, 2), ""),
				"Test the job creation function.",
				*store,
			).unwrap();

			let test_org: Result<Organization, Box<dyn Error>> = test_job.into();

			assert_eq!(dogood.organization, test_org.unwrap());

			println!("\n>>>>> BincodeJob test_into_organization {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		}).unwrap();
	}
}

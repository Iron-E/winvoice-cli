use
{
	crate::data::{BincodeJob, BincodeOrganization, Result},
	clinvoice_adapter::data::{Error as DataError, MatchWhen, OrganizationAdapter},
	clinvoice_data::Organization,
};

impl Into<Result<Organization>> for BincodeJob<'_>
{
	fn into(self) -> Result<Organization>
	{
		let results = BincodeOrganization::retrieve(
			MatchWhen::EqualTo(self.job.client_id), // id
			MatchWhen::Any, // location
			MatchWhen::Any, // name
			self.store,
		)?;

		let organization = match results.get(0)
		{
			Some(org) => org,
			_ => return Err(DataError::DataIntegrity {id: self.job.client_id}.into()),
		};

		Ok(organization.clone())
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeJob, BincodeOrganization, OrganizationAdapter, Result},
		crate::util,
		clinvoice_adapter::data::JobAdapter,
		clinvoice_data::{chrono::Utc, Decimal, Id, Location, Money, Organization},
		std::time::Instant,
	};

	#[test]
	fn test_into_organization()
	{
		util::test_temp_store(|store|
		{
			let dogood = BincodeOrganization::create(
				Location {name: "Earth".into(), id: Id::new_v4(), outer_id: None},
				"DoGood Inc",
				&store
			).unwrap();

			let test_job = BincodeJob
			{
				job: BincodeJob::create(
					dogood.clone(),
					Utc::now(),
					Money::new(Decimal::new(200, 2), ""),
					"Test the job creation function.",
					&store,
				).unwrap(),
				store,
			};

			let start = Instant::now();
			let test_org: Result<Organization> = test_job.into();
			println!("\n>>>>> BincodeJob::into_organization {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

			assert_eq!(dogood, test_org.unwrap());
		});
	}
}

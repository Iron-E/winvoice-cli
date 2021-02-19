use
{
	crate::data::{BincodeEmployee, BincodeOrganization},
	clinvoice_adapter::
	{
		data::{Error as DataError, MatchWhen, OrganizationAdapter},
		DynamicResult,
	},
	clinvoice_data::Organization,
};

impl Into<DynamicResult<Organization>> for BincodeEmployee<'_, '_, '_>
{
	fn into(self) -> DynamicResult<Organization>
	{
		let results = BincodeOrganization::retrieve(
			MatchWhen::EqualTo(self.employee.organization_id), // id
			MatchWhen::Any, // location
			MatchWhen::Any, // name
			self.store,
		)?;

		let bincode_organization = match results.iter().next()
		{
			Some(bin_org) => bin_org,
			_ => Err(DataError::DataIntegrity {id: self.employee.organization_id})?,
		};

		return Ok(bincode_organization.organization.clone());
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeEmployee, BincodeOrganization, DynamicResult, OrganizationAdapter},
		crate::util,
		clinvoice_adapter::data::EmployeeAdapter,
		clinvoice_data::{Contact, EmployeeStatus, Id, Location, Organization, Person},
		std::time::Instant,
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
				*store
			).unwrap();

			let testy = BincodeEmployee::create(
				vec![Contact::Email("foo".into())],
				dogood.organization.clone(),
				Person
				{
					contact_info: vec![Contact::Email("yum".into())],
					id: Id::new_v4(),
					name: "Testy Mćtesterson".into(),
				},
				"CEO of Tests",
				EmployeeStatus::Employed,
				*store,
			).unwrap();

			let testy_org: DynamicResult<Organization> = testy.into();

			assert_eq!(dogood.organization, testy_org.unwrap());

			println!("\n>>>>> BincodeEmployee test_into_organization {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}
}

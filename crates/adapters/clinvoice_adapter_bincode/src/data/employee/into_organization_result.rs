use crate::data::{BincodeEmployee, BincodeOrganization};
use clinvoice_adapter::data::{Error as DataError, MatchWhen, OrganizationAdapter};
use clinvoice_data::Organization;
use std::error::Error;

impl Into<Result<Organization, Box<dyn Error>>> for BincodeEmployee<'_, '_, '_>
{
	fn into(self) -> Result<Organization, Box<dyn Error>>
	{
		let results = BincodeOrganization::retrieve(
			MatchWhen::EqualTo(self.employee.organization_id),
			MatchWhen::Any,
			MatchWhen::Any,
			MatchWhen::Any,
			self.store,
		)?;

		let bincode_organization = match results.iter().next()
		{
			Some(bin_org) => bin_org,
			None => Err(DataError::DataIntegrity {id: self.employee.organization_id})?,
		};

		return Ok(bincode_organization.organization.clone());
	}
}

#[cfg(test)]
mod tests
{
	use super::{BincodeEmployee, BincodeOrganization, OrganizationAdapter};
	use crate::util;
	use clinvoice_adapter::data::EmployeeAdapter;
	use clinvoice_data::{Contact, Id, Location, Organization, Person};
	use std::{collections::HashSet, error::Error, time::Instant};

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

			let testy = BincodeEmployee::create(
				[Contact::Email("foo".into())].iter().cloned().collect(),
				dogood.organization.clone(),
				Person
				{
					contact_info: [Contact::Email("yum".into())].iter().cloned().collect(),
					id: Id::new_v4(),
					name: "Testy Mćtesterson".into(),
				},
				*store,
				"CEO of Tests",
			).unwrap();

			let testy_org: Result<Organization, Box<dyn Error>> = testy.into();

			assert_eq!(dogood.organization, testy_org.unwrap());

			println!("\n>>>>> BincodeEmployee test_into_organization {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		}).unwrap();
	}
}

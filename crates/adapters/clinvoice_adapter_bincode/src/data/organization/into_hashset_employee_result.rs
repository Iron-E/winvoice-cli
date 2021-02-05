use
{
	crate::data::{BincodeEmployee, BincodeOrganization},
	clinvoice_adapter::data::{EmployeeAdapter, MatchWhen},
	clinvoice_data::Employee,
	std::{collections::HashSet, error::Error},
};

impl Into<Result<HashSet<Employee>, Box<dyn Error>>> for BincodeOrganization<'_, '_, '_>
{
	fn into(self) -> Result<HashSet<Employee>, Box<dyn Error>>
	{
		let results = BincodeEmployee::retrieve(
			MatchWhen::Any, // contact info
			MatchWhen::Any, // employed
			MatchWhen::Any, // id
			MatchWhen::EqualTo(self.organization.id), // organization
			MatchWhen::Any, // person
			MatchWhen::Any, // title
			self.store,
		)?;

		return Ok(results.iter().map(|result| result.employee.clone()).collect());
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeEmployee, BincodeOrganization, EmployeeAdapter},
		crate::util,
		clinvoice_adapter::data::{OrganizationAdapter, Updatable},
		clinvoice_data::{Contact, Employee, Id, Location, Person},
		std::{collections::HashSet, error::Error, time::Instant},
	};

	#[test]
	fn test_into_hashset_employee()
	{
		let start = Instant::now();

		util::test_temp_store(|store|
		{
			let mut dogood = BincodeOrganization::create(
				Location {name: "Earth".into(), id: Id::new_v4(), outer_id: None},
				"DoGood Inc",
				HashSet::new(),
				*store
			).unwrap();

			let testy = BincodeEmployee::create(
				[Contact::Email("foo@bar.io".into())].iter().cloned().collect(),
				dogood.organization.clone(),
				Person
				{
					contact_info: [Contact::Email("yum@bar.io".into())].iter().cloned().collect(),
					id: Id::new_v4(),
					name: "Testy Mćtesterson".into(),
				},
				"CEO of Tests",
				*store,
			).unwrap();

			let mr_flu = BincodeEmployee::create(
				[Contact::Email("flu@bar.io".into())].iter().cloned().collect(),
				dogood.organization.clone(),
				Person
				{
					contact_info: [Contact::Email("sig@bar.io".into())].iter().cloned().collect(),
					id: Id::new_v4(),
					name: "Mr. Flu".into(),
				},
				"Janitor",
				*store,
			).unwrap();

			// Insert the new hired employees
			dogood.organization.representatives.insert(testy.employee.id);
			dogood.organization.representatives.insert(mr_flu.employee.id);
			dogood.update().unwrap();

			// Retrieve the written employees back into the `Employee` structure.
			let reps: Result<HashSet<Employee>, Box<dyn Error>> = dogood.into();

			assert_eq!(reps.unwrap(), [testy.employee, mr_flu.employee].iter().cloned().collect());

			println!("\n>>>>> BincodeOrganization test_into_hashset_employee {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		}).unwrap();
	}
}

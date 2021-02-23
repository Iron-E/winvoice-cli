use
{
	crate::data::{BincodeEmployee, BincodeOrganization},
	clinvoice_adapter::
	{
		data::{EmployeeAdapter, MatchWhen},
		DynamicResult,
	},
	clinvoice_data::Employee,
};

impl Into<DynamicResult<Vec<Employee>>> for BincodeOrganization<'_, '_, '_>
{
	fn into(self) -> DynamicResult<Vec<Employee>>
	{
		let results = BincodeEmployee::retrieve(
			MatchWhen::Any, // contact info
			MatchWhen::Any, // id
			MatchWhen::EqualTo(self.organization.id), // organization
			MatchWhen::Any, // person
			MatchWhen::Any, // status
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
		clinvoice_adapter::{data::OrganizationAdapter, DynamicResult},
		clinvoice_data::{Contact, Employee, EmployeeStatus, Id, Location, Person},
		std::{collections::HashSet, time::Instant},
	};

	#[test]
	fn test_into_vec_employee()
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
				vec![Contact::Email("foo@bar.io".into())],
				dogood.organization.clone(),
				Person
				{
					contact_info: vec![Contact::Email("yum@bar.io".into())],
					id: Id::new_v4(),
					name: "Testy Mćtesterson".into(),
				},
				"CEO of Tests",
				EmployeeStatus::Representative,
				*store,
			).unwrap();

			let mr_flu = BincodeEmployee::create(
				vec![Contact::Email("flu@bar.io".into())],
				dogood.organization.clone(),
				Person
				{
					contact_info: vec![Contact::Email("sig@bar.io".into())],
					id: Id::new_v4(),
					name: "Mr. Flu".into(),
				},
				"Janitor",
				EmployeeStatus::Employed,
				*store,
			).unwrap();

			// Retrieve the written employees back into the `Employee` structure.
			let reps: DynamicResult<Vec<Employee>> = dogood.into();

			assert_eq!(
				reps.unwrap().into_iter().collect::<HashSet<Employee>>(),
				[mr_flu.employee, testy.employee].iter().cloned().collect::<HashSet<Employee>>()
			);

			println!("\n>>>>> BincodeOrganization::into_vec_employee {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}
}

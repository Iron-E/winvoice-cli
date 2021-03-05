use
{
	crate::data::{BincodeEmployee, BincodeOrganization, Result},
	clinvoice_adapter::data::{EmployeeAdapter, MatchWhen},
	clinvoice_data::Employee,
};

impl Into<Result<Vec<Employee>>> for BincodeOrganization<'_, '_>
{
	fn into(self) -> Result<Vec<Employee>>
	{
		Ok(BincodeEmployee::retrieve(
			MatchWhen::Any, // contact info
			MatchWhen::Any, // id
			MatchWhen::EqualTo(self.organization.id), // organization
			MatchWhen::Any, // person
			MatchWhen::Any, // status
			MatchWhen::Any, // title
			self.store,
		)?)
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeEmployee, BincodeOrganization, EmployeeAdapter, Result},
		crate::util,
		clinvoice_adapter::data::OrganizationAdapter,
		clinvoice_data::{Contact, Employee, EmployeeStatus, Id, Location, Person},
		std::{collections::HashSet, time::Instant},
	};

	#[test]
	fn test_into_vec_employee()
	{
		util::test_temp_store(|store|
		{
			let dogood = BincodeOrganization
			{
				organization: &BincodeOrganization::create(
					Location {name: "Earth".into(), id: Id::new_v4(), outer_id: None},
					"DoGood Inc",
					&store
				).unwrap(),
				store,
			};

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
				&store,
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
				&store,
			).unwrap();

			let start = Instant::now();
			// Retrieve the written employees back into the `Employee` structure.
			let reps: Result<Vec<Employee>> = dogood.into();
			println!("\n>>>>> BincodeOrganization::into_vec_employee {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

			assert_eq!(
				reps.unwrap().into_iter().collect::<HashSet<Employee>>(),
				[mr_flu, testy].iter().cloned().collect::<HashSet<Employee>>()
			);
		});
	}
}

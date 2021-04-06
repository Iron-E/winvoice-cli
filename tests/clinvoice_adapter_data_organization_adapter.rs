mod util;

use
{
	std::{collections::HashSet, time::Instant},

	clinvoice_adapter::data::{EmployeeAdapter, LocationAdapter, OrganizationAdapter},
	clinvoice_adapter_bincode::data::{BincodeEmployee, BincodeLocation, BincodeOrganization},
	clinvoice_data::{Contact, Id, EmployeeStatus, Location, Person},
};

#[test]
fn into_location()
{
	util::temp_store(|store|
	{
		let arizona = BincodeLocation::create("Arizona", &store).unwrap();
		let dogood = BincodeOrganization::create(
			arizona.clone(),
			"DoGood Inc",
			&store
		).unwrap();

		let start = Instant::now();
		// Retrieve the written employees back into the `Employee` structure.
		let dogood_location = BincodeOrganization::into_location::<BincodeLocation>(&dogood, store);
		println!("\n>>>>> BincodeOrganization::into_location {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

		// Assert that the location retrieved is the location expected
		assert_eq!(arizona, dogood_location.unwrap());
	});
}

#[test]
fn into_vec_employee()
{
	util::temp_store(|store|
	{
		let dogood = BincodeOrganization::create(
			Location {name: "Earth".into(), id: Id::new_v4(), outer_id: None},
			"DoGood Inc",
			&store
		).unwrap();

		let testy = BincodeEmployee::create(
			vec![("Work Email".into(), Contact::Email("foo@bar.io".into()))].into_iter().collect(),
			dogood.clone(),
			Person
			{
				id: Id::new_v4(),
				name: "Testy Mćtesterson".into(),
			},
			EmployeeStatus::Representative,
			"CEO of Tests",
			&store,
		).unwrap();

		let mr_flu = BincodeEmployee::create(
			vec![("Work Email".into(), Contact::Email("flu@bar.io".into()))].into_iter().collect(),
			dogood.clone(),
			Person
			{
				id: Id::new_v4(),
				name: "Mr. Flu".into(),
			},
			EmployeeStatus::Employed,
			"Janitor",
			&store,
		).unwrap();

		let start = Instant::now();
		// Retrieve the written employees back into the `Employee` structure.
		let reps = BincodeOrganization::into_employees::<BincodeEmployee>(&dogood, store);
		println!("\n>>>>> BincodeOrganization::into_vec_employee {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

		assert_eq!(
			reps.unwrap().into_iter().collect::<HashSet<_>>(),
			[mr_flu, testy].iter().cloned().collect::<HashSet<_>>()
		);
	});
}

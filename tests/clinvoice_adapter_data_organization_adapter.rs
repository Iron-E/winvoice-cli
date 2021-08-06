mod util;

use std::{collections::HashSet, time::Instant};

use clinvoice_adapter::data::{EmployeeAdapter, LocationAdapter, OrganizationAdapter};
use clinvoice_adapter_bincode::data::{BincodeEmployee, BincodeLocation, BincodeOrganization};
use clinvoice_data::{Contact, EmployeeStatus, Id, Location, Person};

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn to_location()
{
	let store = util::temp_store();

	let arizona = BincodeLocation::create("Arizona".into(), &store)
		.await
		.unwrap();
	let dogood = BincodeOrganization::create(arizona.clone(), "DoGood Inc".into(), &store)
		.await
		.unwrap();

	let start = Instant::now();
	// Retrieve the written employees back into the `Employee` structure.
	let dogood_location = BincodeOrganization::to_location::<BincodeLocation>(&dogood, &store).await;
	println!(
		"\n>>>>> BincodeOrganization::to_location {}us <<<<<\n",
		Instant::now().duration_since(start).as_micros()
	);

	// Assert that the location retrieved is the location expected
	assert_eq!(arizona, dogood_location.unwrap());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn to_vec_employee()
{
	let store = util::temp_store();

	let dogood = BincodeOrganization::create(
		Location {
			name: "Earth".into(),
			id: Id::new_v4(),
			outer_id: None,
		},
		"DoGood Inc".into(),
		&store,
	)
	.await
	.unwrap();

	let testy = BincodeEmployee::create(
		vec![("Work Email".into(), Contact::Email {
			email:  "foo@bar.io".into(),
			export: false,
		})]
		.into_iter()
		.collect(),
		dogood.clone(),
		Person {
			id:   Id::new_v4(),
			name: "Testy Mćtesterson".into(),
		},
		EmployeeStatus::Representative,
		"CEO of Tests".into(),
		&store,
	)
	.await
	.unwrap();

	let mr_flu = BincodeEmployee::create(
		vec![("Work Email".into(), Contact::Email {
			email:  "flu@bar.io".into(),
			export: false,
		})]
		.into_iter()
		.collect(),
		dogood.clone(),
		Person {
			id:   Id::new_v4(),
			name: "Mr. Flu".into(),
		},
		EmployeeStatus::Employed,
		"Janitor".into(),
		&store,
	)
	.await
	.unwrap();

	let start = Instant::now();
	// Retrieve the written employees back into the `Employee` structure.
	let reps = BincodeOrganization::to_employees::<BincodeEmployee>(&dogood, &store).await;
	println!(
		"\n>>>>> BincodeOrganization::to_vec_employee {}us <<<<<\n",
		Instant::now().duration_since(start).as_micros()
	);

	assert_eq!(
		reps.unwrap().into_iter().collect::<HashSet<_>>(),
		[mr_flu, testy].iter().cloned().collect::<HashSet<_>>()
	);
}

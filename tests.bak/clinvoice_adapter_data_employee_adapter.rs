mod util;

use clinvoice_adapter::schema::{
	EmployeeAdapter,
	LocationAdapter,
	OrganizationAdapter,
	PersonAdapter,
};
use clinvoice_adapter_bincode::schema::{
	BincodeEmployee,
	BincodeLocation,
	BincodeOrganization,
	BincodePerson,
};
use clinvoice_schema::{
	Employee,
	Contact,
	Id,
	Location,
	Organization,
	Person,
};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn into_view()
{
	let store = util::temp_store();

	let earth = BincodeLocation::create("Earth".into(), &store)
		.await
		.unwrap();

	let big_old_test =
		BincodeOrganization::create(earth.clone(), "Big Old Test Corporation".into(), &store)
			.await
			.unwrap();

	let testy = BincodePerson::create("Testy Mćtesterson".into(), &store)
		.await
		.unwrap();

	let ceo_testy = BincodeEmployee::create(
		vec![("Work".into(), Contact::Address {
			location_id: earth.id,
			export:      false,
		})]
		.into_iter()
		.collect(),
		big_old_test.clone(),
		testy.clone(),
		"Employed".into(),
		"CEO of Tests".into(),
		&store,
	)
	.await
	.unwrap();

	let earth_view = Location {
		id:    earth.id,
		name:  earth.name,
		outer: None,
	};

	let ceo_testy_view = Employee {
		contact_info: vec![("Work".into(), Contact::Address {
			location: earth_view.clone(),
			export:   false,
		})]
		.into_iter()
		.collect(),
		id: ceo_testy.id,
		organization: Organization {
			id: big_old_test.id,
			location: earth_view.clone(),
			name: big_old_test.name,
		},
		person: Person {
			id:   testy.id,
			name: testy.name,
		},
		title: ceo_testy.title.clone(),
		status: ceo_testy.status,
	};

	let ceo_testy_view_result =
		BincodeEmployee::into_view::<BincodeLocation, BincodeOrganization, BincodePerson>(
			ceo_testy, &store,
		)
		.await;

	// Asser that the synthetic view is the same as the view which was created naturally.
	assert_eq!(ceo_testy_view, ceo_testy_view_result.unwrap());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn to_organization()
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
			email:  "foo".into(),
			export: false,
		})]
		.into_iter()
		.collect(),
		dogood.clone(),
		Person {
			id:   Id::new_v4(),
			name: "Testy Mćtesterson".into(),
		},
		"Employed".into(),
		"CEO of Tests".into(),
		&store,
	)
	.await
	.unwrap();

	let testy_org = BincodeEmployee::to_organization::<BincodeOrganization>(&testy, &store).await;

	assert_eq!(dogood, testy_org.unwrap());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn to_person()
{
	let store = util::temp_store();

	let testy = BincodePerson::create("Testy Mćtesterson".into(), &store)
		.await
		.unwrap();

	let testy_employed = BincodeEmployee::create(
		vec![("Work Email".into(), Contact::Email {
			email:  "foo".into(),
			export: false,
		})]
		.into_iter()
		.collect(),
		Organization {
			id: Id::new_v4(),
			location_id: Id::new_v4(),
			name: "DoGood Inc".into(),
		},
		testy.clone(),
		"Not employed".into(),
		"CEO of Tests".into(),
		&store,
	)
	.await
	.unwrap();

	let testy_person = BincodeEmployee::to_person::<BincodePerson>(&testy_employed, &store).await;

	assert_eq!(testy, testy_person.unwrap());
}

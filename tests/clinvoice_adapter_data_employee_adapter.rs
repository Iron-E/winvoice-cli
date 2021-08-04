mod util;

use std::time::Instant;

use clinvoice_adapter::data::{
	EmployeeAdapter,
	LocationAdapter,
	OrganizationAdapter,
	PersonAdapter,
};
use clinvoice_adapter_bincode::data::{
	BincodeEmployee,
	BincodeLocation,
	BincodeOrganization,
	BincodePerson,
};
use clinvoice_data::{
	views::{
		ContactView,
		EmployeeView,
		LocationView,
		OrganizationView,
		PersonView,
	},
	Contact,
	EmployeeStatus,
	Id,
	Location,
	Organization,
	Person,
};

#[test]
fn to_organization()
{
	util::temp_store(|store| {
		let dogood = BincodeOrganization::create(
			Location {
				name: "Earth".into(),
				id: Id::new_v4(),
				outer_id: None,
			},
			"DoGood Inc".into(),
			&store,
		)
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
			EmployeeStatus::Employed,
			"CEO of Tests".into(),
			&store,
		)
		.unwrap();

		let start = Instant::now();
		let testy_org = BincodeEmployee::to_organization::<BincodeOrganization>(&testy, store);
		println!(
			"\n>>>>> BincodeEmployee::to_organization {}us <<<<<\n",
			Instant::now().duration_since(start).as_micros()
		);

		assert_eq!(dogood, testy_org.unwrap());
	});
}

#[test]
fn to_person()
{
	util::temp_store(|store| {
		let testy = BincodePerson::create("Testy Mćtesterson".into(), &store).unwrap();

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
			EmployeeStatus::NotEmployed,
			"CEO of Tests".into(),
			&store,
		)
		.unwrap();

		let start = Instant::now();
		let testy_person = BincodeEmployee::to_person::<BincodePerson>(&testy_employed, store);
		println!(
			"\n>>>>> BincodeEmployee::to_person {}us <<<<<\n",
			Instant::now().duration_since(start).as_micros()
		);

		assert_eq!(testy, testy_person.unwrap());
	});
}

#[test]
fn to_view()
{
	util::temp_store(|store| {
		let earth = BincodeLocation::create("Earth".into(), &store).unwrap();

		let big_old_test =
			BincodeOrganization::create(earth.clone(), "Big Old Test Corporation".into(), &store)
				.unwrap();

		let testy = BincodePerson::create("Testy Mćtesterson".into(), &store).unwrap();

		let ceo_testy = BincodeEmployee::create(
			vec![("Work".into(), Contact::Address {
				location_id: earth.id,
				export:      false,
			})]
			.into_iter()
			.collect(),
			big_old_test.clone(),
			testy.clone(),
			EmployeeStatus::Employed,
			"CEO of Tests".into(),
			&store,
		)
		.unwrap();

		let earth_view = LocationView {
			id:    earth.id,
			name:  earth.name,
			outer: None,
		};

		let ceo_testy_view = EmployeeView {
			contact_info: vec![("Work".into(), ContactView::Address {
				location: earth_view.clone(),
				export:   false,
			})]
			.into_iter()
			.collect(),
			id: ceo_testy.id,
			organization: OrganizationView {
				id: big_old_test.id,
				location: earth_view.clone(),
				name: big_old_test.name,
			},
			person: PersonView {
				id:   testy.id,
				name: testy.name,
			},
			title: ceo_testy.title.clone(),
			status: ceo_testy.status,
		};

		let start = Instant::now();
		let ceo_testy_view_result =
			BincodeEmployee::into_view::<BincodeLocation, BincodeOrganization, BincodePerson>(
				ceo_testy, store,
			);
		println!(
			"\n>>>>> BincodeEmployee::to_view {}us <<<<<\n",
			Instant::now().duration_since(start).as_micros()
		);

		// Asser that the synthetic view is the same as the view which was created naturally.
		assert_eq!(ceo_testy_view, ceo_testy_view_result.unwrap());
	});
}

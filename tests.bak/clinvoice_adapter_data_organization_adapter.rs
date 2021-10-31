mod util;

use std::collections::HashSet;

use clinvoice_adapter::data::{EmployeeAdapter, LocationAdapter, OrganizationAdapter};
use clinvoice_adapter_bincode::data::{BincodeEmployee, BincodeLocation, BincodeOrganization};
use clinvoice_data::{views::OrganizationView, Contact, EmployeeStatus, Id, Location, Person};
#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn into_view()
{
	let store = util::temp_store();

	let earth = BincodeLocation::create("Earth".into(), &store)
		.await
		.unwrap();

	let usa = BincodeLocation {
		location: &earth,
		store:    &store,
	}
	.create_inner("USA".into())
	.await
	.unwrap();

	let arizona = BincodeLocation {
		location: &usa,
		store:    &store,
	}
	.create_inner("Arizona".into())
	.await
	.unwrap();

	let phoenix = BincodeLocation {
		location: &arizona,
		store:    &store,
	}
	.create_inner("Phoenix".into())
	.await
	.unwrap();

	let (alsd, eal, aaa, focj, giguy) = futures::try_join!(
		BincodeOrganization::create(earth.clone(), "alsdkjaldkj".into(), &store),
		BincodeOrganization::create(usa.clone(), "alskdjalgkh  ladhkj EAL ISdh".into(), &store),
		BincodeOrganization::create(arizona.clone(), " AAA – 44 %%".into(), &store),
		BincodeOrganization::create(phoenix.clone(), " ^^^ ADSLKJDLASKJD FOCJCI".into(), &store),
		BincodeOrganization::create(phoenix.clone(), "aldkj doiciuc giguy &&".into(), &store),
	)
	.unwrap();

	let (alsd_view, eal_view, aaa_view, focj_view, giguy_view) = futures::try_join!(
		BincodeOrganization::into_view::<BincodeLocation>(alsd.clone(), &store),
		BincodeOrganization::into_view::<BincodeLocation>(eal.clone(), &store),
		BincodeOrganization::into_view::<BincodeLocation>(aaa.clone(), &store),
		BincodeOrganization::into_view::<BincodeLocation>(focj.clone(), &store),
		BincodeOrganization::into_view::<BincodeLocation>(giguy.clone(), &store),
	)
	.unwrap();

	let phoenix_view = BincodeLocation::into_view(phoenix, &store).await.unwrap();

	assert_eq!(alsd_view, OrganizationView {
		id: alsd.id,
		location: BincodeLocation::into_view(earth, &store).await.unwrap(),
		name: alsd.name,
	});

	assert_eq!(eal_view, OrganizationView {
		id: eal.id,
		location: BincodeLocation::into_view(usa, &store).await.unwrap(),
		name: eal.name,
	});

	assert_eq!(aaa_view, OrganizationView {
		id: aaa.id,
		location: BincodeLocation::into_view(arizona, &store).await.unwrap(),
		name: aaa.name,
	});

	assert_eq!(focj_view, OrganizationView {
		id: focj.id,
		location: phoenix_view.clone(),
		name: focj.name,
	});

	assert_eq!(giguy_view, OrganizationView {
		id: giguy.id,
		location: phoenix_view,
		name: giguy.name,
	});
}

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

	// Retrieve the written employees back into the `Employee` structure.
	let dogood_location = BincodeOrganization::to_location::<BincodeLocation>(&dogood, &store).await;

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

	// Retrieve the written employees back into the `Employee` structure.
	let reps = BincodeOrganization::to_employees::<BincodeEmployee>(&dogood, &store).await;

	assert_eq!(
		reps.unwrap().into_iter().collect::<HashSet<_>>(),
		[mr_flu, testy].iter().cloned().collect::<HashSet<_>>()
	);
}

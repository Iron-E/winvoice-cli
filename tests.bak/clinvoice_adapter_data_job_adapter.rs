mod util;

use std::{collections::HashMap, time::Instant};

use clinvoice_adapter::data::{
	EmployeeAdapter,
	JobAdapter,
	LocationAdapter,
	OrganizationAdapter,
	PersonAdapter,
};
use clinvoice_adapter_bincode::data::{
	BincodeEmployee,
	BincodeJob,
	BincodeLocation,
	BincodeOrganization,
	BincodePerson,
};
use clinvoice_data::{
	chrono::Utc,
	Currency,
	Money,
	views::{
		ContactView,
		EmployeeView,
		JobView,
		LocationView,
		OrganizationView,
		PersonView,
		TimesheetView,
	},
	Contact,
	EmployeeStatus,
	Id,
	Location,
};

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn into_view()
{
	let store = util::temp_store();

	let earth = BincodeLocation::create("Earth".into(), &store)
		.await
		.unwrap();

	let big_test =
		BincodeOrganization::create(earth.clone(), "Big Old Test Corporation".into(), &store)
			.await
			.unwrap();

	let mut create_job = BincodeJob::create(
		big_test.clone(),
		Utc::now(),
		Money::new(2_00, 2, Currency::USD),
		"Test the job creation function".into(),
		&store,
	)
	.await
	.unwrap();

	let contact_info: HashMap<_, _> = vec![("Address".into(), Contact::Address {
		location_id: earth.id,
		export:      false,
	})]
	.into_iter()
	.collect();

	let testy = BincodePerson::create("Testy Mćtesterson".into(), &store)
		.await
		.unwrap();

	let ceo_testy = BincodeEmployee::create(
		contact_info.clone(),
		big_test.clone(),
		testy.clone(),
		EmployeeStatus::Employed,
		"CEO of Tests".into(),
		&store,
	)
	.await
	.unwrap();

	let earth_view = LocationView {
		id:    earth.id,
		name:  earth.name,
		outer: None,
	};

	let contact_info_view: HashMap<String, ContactView> =
		vec![("Address View".into(), ContactView::Address {
			location: earth_view.clone(),
			export:   false,
		})]
		.into_iter()
		.collect();

	let ceo_testy_view = EmployeeView {
		contact_info: contact_info_view.clone(),
		id: ceo_testy.id,
		organization: OrganizationView {
			id: big_test.id,
			location: earth_view,
			name: big_test.name,
		},
		person: PersonView {
			id:   testy.id,
			name: testy.name,
		},
		title: ceo_testy.title.clone(),
		status: ceo_testy.status,
	};

	create_job.start_timesheet(ceo_testy.id);

	let create_job_view = JobView {
		client: ceo_testy_view.organization.clone(),
		date_close: create_job.date_close,
		date_open: create_job.date_open,
		id: create_job.id,
		invoice: create_job.invoice.clone(),
		notes: create_job.notes.clone(),
		objectives: create_job.objectives.clone(),
		timesheets: vec![TimesheetView {
			employee:   ceo_testy_view,
			expenses:   Vec::new(),
			time_begin: create_job
				.timesheets
				.first()
				.expect("Timesheet did not attach!")
				.time_begin,
			time_end:   None,
			work_notes: create_job
				.timesheets
				.first()
				.expect("Timesheet did not attach!")
				.work_notes
				.clone(),
		}],
	};

	let start = Instant::now();
	let create_job_view_result = BincodeJob::into_view::<
		BincodeEmployee,
		BincodeLocation,
		BincodeOrganization,
		BincodePerson,
	>(create_job, &store)
	.await;
	println!(
		"\n>>>>> BincodeJob::into_view {}us <<<<<\n",
		Instant::now().duration_since(start).as_micros()
	);

	assert_eq!(create_job_view, create_job_view_result.unwrap());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
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

	let test_job = BincodeJob::create(
		dogood.clone(),
		Utc::now(),
		Money::new(2_00, 2, Currency::USD),
		"Test the job creation function".into(),
		&store,
	)
	.await
	.unwrap();

	let start = Instant::now();
	let test_org = BincodeJob::to_organization::<BincodeOrganization>(&test_job, &store).await;
	println!(
		"\n>>>>> BincodeJob::to_organization {}us <<<<<\n",
		Instant::now().duration_since(start).as_micros()
	);

	assert_eq!(dogood, test_org.unwrap());
}

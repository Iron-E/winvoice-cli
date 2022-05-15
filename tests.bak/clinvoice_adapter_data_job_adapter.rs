mod util;

use std::collections::HashMap;

use clinvoice_adapter::schema::{
	EmployeeAdapter,
	JobAdapter,
	LocationAdapter,
	OrganizationAdapter,
	PersonAdapter,
};
use clinvoice_adapter_bincode::schema::{
	BincodeEmployee,
	BincodeJob,
	BincodeLocation,
	BincodeOrganization,
	BincodePerson,
};
use clinvoice_schema::{
	chrono::Utc,
	Contact,
	Currency,
	Employee,
	Id,
	Job,
	Location,
	Money,
	Organization,
	Person,
	Timesheet,
};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
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

	let contact_info = vec![Contact::Address {
		location_id: earth.id,
		label: "Address".into(),
		export:      false,
	}];

	let testy = BincodePerson::create("Testy Mćtesterson".into(), &store)
		.await
		.unwrap();

	let ceo_testy = BincodeEmployee::create(
		contact_info.clone(),
		big_test.clone(),
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

	let contact_info_view = vec![Contact::Address {
		location: earth_view.clone(),
		label: "Address ".into(),
		export:   false,
	}];

	let ceo_testy_view = Employee {
		contact_info: contact_info_view.clone(),
		id: ceo_testy.id,
		organization: Organization {
			id: big_test.id,
			location: earth_view,
			name: big_test.name,
		},
		person: Person {
			id:   testy.id,
			name: testy.name,
		},
		title: ceo_testy.title.clone(),
		status: ceo_testy.status,
	};

	create_job.start_timesheet(ceo_testy.id);

	let create_job_view = Job {
		client: ceo_testy_view.organization.clone(),
		date_close: create_job.date_close,
		date_open: create_job.date_open,
		id: create_job.id,
		invoice: create_job.invoice.clone(),
		notes: create_job.notes.clone(),
		objectives: create_job.objectives.clone(),
		timesheets: vec![Timesheet {
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

	let create_job_view_result = BincodeJob::into_view::<
		BincodeEmployee,
		BincodeLocation,
		BincodeOrganization,
		BincodePerson,
	>(create_job, &store)
	.await;

	assert_eq!(create_job_view, create_job_view_result.unwrap());
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

	let test_job = BincodeJob::create(
		dogood.clone(),
		Utc::now(),
		Money::new(2_00, 2, Currency::USD),
		"Test the job creation function".into(),
		&store,
	)
	.await
	.unwrap();

	let test_org = BincodeJob::to_organization::<BincodeOrganization>(&test_job, &store).await;

	assert_eq!(dogood, test_org.unwrap());
}

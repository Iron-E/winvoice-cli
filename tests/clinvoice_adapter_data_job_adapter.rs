mod util;

use
{
	clinvoice_adapter::data::{EmployeeAdapter, JobAdapter, LocationAdapter, OrganizationAdapter, PersonAdapter},
	clinvoice_adapter_bincode::data::{BincodeEmployee, BincodeJob, BincodeLocation, BincodeOrganization, BincodePerson},
	clinvoice_data::
	{
		chrono::Utc,
		Contact, Decimal, EmployeeStatus, Id, Location, Money,
		views::{ContactView, EmployeeView, JobView, LocationView, OrganizationView, PersonView, TimesheetView},
	},
	std::{collections::HashMap, time::Instant},
};

#[test]
fn into_organization()
{
	util::temp_store(|store|
	{
		let dogood = BincodeOrganization::create(
			Location {name: "Earth".into(), id: Id::new_v4(), outer_id: None},
			"DoGood Inc",
			&store
		).unwrap();

		let test_job = BincodeJob::create(
			dogood.clone(),
			Utc::now(),
			Money::new(Decimal::new(200, 2), ""),
			"Test the job creation function",
			&store,
		).unwrap();

		let start = Instant::now();
		let test_org = BincodeJob::into_organization::<BincodeOrganization>(&test_job, store);
		println!("\n>>>>> BincodeJob::into_organization {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

		assert_eq!(dogood, test_org.unwrap());
	});
}

#[test]
fn into_view()
{
	util::temp_store(|store|
	{
		let earth = BincodeLocation::create(
			"Earth".into(),
			&store,
		).unwrap();

		let big_test = BincodeOrganization::create(
			earth.clone(),
			"Big Old Test Corporation".into(),
			&store,
		).unwrap();

		let mut create_job = BincodeJob::create(
			big_test.clone(),
			Utc::now(),
			Money::new(Decimal::new(200, 2), ""),
			"Test the job creation function",
			&store,
		).unwrap();

		let contact_info: HashMap<_, _> = vec![
			("Address".into(), Contact::Address(earth.id))
		].into_iter().collect();

		let testy = BincodePerson::create(
			"Testy Mćtesterson",
			&store,
		).unwrap();

		let ceo_testy = BincodeEmployee::create(
			contact_info.clone(),
			big_test.clone(),
			testy.clone(),
			EmployeeStatus::Employed,
			"CEO of Tests",
			&store,
		).unwrap();

		let earth_view = LocationView
		{
			id: earth.id,
			name: earth.name,
			outer: None,
		};

		let contact_info_view: HashMap<String, ContactView> = vec![
			("Address View".into(), earth_view.clone().into())
		].into_iter().collect();

		let ceo_testy_view = EmployeeView
		{
			contact_info: contact_info_view.clone(),
			id: ceo_testy.id,
			organization: OrganizationView
			{
				id: big_test.id,
				location: earth_view,
				name: big_test.name,
			},
			person: PersonView
			{
				id: testy.id,
				name: testy.name,
			},
			title: ceo_testy.title.clone(),
			status: ceo_testy.status,
		};

		create_job.start_timesheet(ceo_testy.id);

		let create_job_view = JobView
		{
			client: ceo_testy_view.organization.clone(),
			date_close: create_job.date_close,
			date_open: create_job.date_open,
			id: create_job.id,
			invoice: create_job.invoice.clone(),
			notes: create_job.notes.clone(),
			objectives: create_job.objectives.clone(),
			timesheets: vec![TimesheetView
			{
				employee: ceo_testy_view,
				expenses: None,
				time_begin: create_job.timesheets.first().expect("Timesheet did not attach!").time_begin,
				time_end: None,
				work_notes: create_job.timesheets.first().expect("Timesheet did not attach!").work_notes.clone(),
			}],
		};

		let start = Instant::now();
		let create_job_view_result = BincodeJob::into_view::<BincodeEmployee, BincodeLocation, BincodeOrganization, BincodePerson>(create_job, store);
		println!("\n>>>>> BincodeJob::into_view {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

		assert_eq!(create_job_view, create_job_view_result.unwrap());
	});
}

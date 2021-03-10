use
{
	super::BincodeJob,
	crate::data::{BincodeEmployee, BincodeOrganization, Result},
	clinvoice_adapter::data::{EmployeeAdapter, Error, MatchWhen},
	clinvoice_data::
	{
		Organization,
		views::{EmployeeView, JobView, OrganizationView, TimesheetView},
	},
};

impl Into<Result<JobView>> for BincodeJob<'_, '_>
{
	fn into(self) -> Result<JobView>
	{
		let date_close = self.job.date_close;
		let date_open = self.job.date_open;
		let id = self.job.id;
		let invoice = self.job.invoice.clone();
		let notes = self.job.notes.clone();
		let objectives = self.job.objectives.clone();
		let store = self.store;
		let timesheets = self.job.timesheets.clone();

		let organization_result: Result<Organization> = self.into();
		let organization_view_result: Result<OrganizationView> = BincodeOrganization
		{
			organization: &organization_result?,
			store,
		}.into();

		let mut timesheet_views = Vec::<TimesheetView>::with_capacity(timesheets.len());

		for timesheet in timesheets
		{
			let employee_view_result: Result<EmployeeView> = match BincodeEmployee::retrieve(
				MatchWhen::Any, // contact_info
				MatchWhen::EqualTo(timesheet.employee_id), // id
				MatchWhen::Any, // organization
				MatchWhen::Any, // person
				MatchWhen::Any, // title
				MatchWhen::Any, // status
				store,
			)?.first()
			{
				Some(first) => BincodeEmployee {employee: first, store}.into(),
				_ => Err(Error::DataIntegrity {id: timesheet.employee_id}.into()),
			};

			timesheet_views.push(TimesheetView
			{
				employee: employee_view_result?,
				expenses: timesheet.expenses,
				time_begin: timesheet.time_begin,
				time_end: timesheet.time_end,
				work_notes: timesheet.work_notes,
			});
		}

		Ok(JobView
		{
			client: organization_view_result?,
			date_close,
			date_open,
			id,
			invoice,
			notes,
			objectives,
			timesheets: timesheet_views,
		})
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeJob, JobView, OrganizationView, TimesheetView, Result},
		crate::
		{
			data::{BincodeEmployee, BincodeLocation, BincodeOrganization, BincodePerson},
			util,
		},
		clinvoice_adapter::data::{EmployeeAdapter, JobAdapter, LocationAdapter, OrganizationAdapter, PersonAdapter},
		clinvoice_data::
		{
			chrono::Utc, Contact, Decimal, EmployeeStatus, Money,
			views::{EmployeeView, LocationView, PersonView},
		},
		std::time::Instant,
	};

	#[test]
	fn test_into_view()
	{
		util::test_temp_store(|store|
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
				"Test the job creation function.",
				&store,
			).unwrap();

			let contact_info = vec![Contact::Address(earth.id)];

			let testy = BincodePerson::create(
				contact_info.clone(),
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

			let contact_info_view = vec![earth_view.clone().into()];

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
					contact_info: contact_info_view,
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
					time_begin: match create_job.timesheets.first()
					{
						Some(t) => t.time_begin,
						_ => panic!("Timesheet did not attach!"),
					},
					time_end: None,
					work_notes: match create_job.timesheets.first()
					{
						Some(t) => t.work_notes.clone(),
						_ => panic!("Timesheet did not attach!"),
					},
				}],
			};

			let start = Instant::now();
			let create_job_view_result: Result<JobView> = BincodeJob {job: &create_job, store}.into();
			println!("\n>>>>> BincodeJob::into_view {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

			assert_eq!(create_job_view, create_job_view_result.unwrap());
		});
	}
}

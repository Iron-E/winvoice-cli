use
{
	super::BincodeJob,
	crate::data::BincodeOrganization,
	clinvoice_adapter::DynamicResult,
	clinvoice_data::
	{
		Organization,
		views::{JobView, OrganizationView},
	},
};

impl Into<DynamicResult<JobView>> for BincodeJob<'_, '_, '_>
{
	fn into(self) -> DynamicResult<JobView>
	{
		let organization_result: DynamicResult<Organization> = self.clone().into();
		let organization_view_result: DynamicResult<OrganizationView> = BincodeOrganization
		{
			organization: organization_result?,
			store: self.store,
		}.into();

		return Ok(JobView
		{
			client: organization_view_result?,
			date_close: self.job.date_close,
			date_open: self.job.date_open,
			invoice: self.job.invoice,
			notes: self.job.notes,
			objectives: self.job.objectives,
			timesheets: self.job.timesheets,
		});
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeJob, DynamicResult, JobView, OrganizationView},
		crate::
		{
			data::{BincodeLocation, BincodeOrganization},
			util,
		},
		clinvoice_adapter::data::{JobAdapter, LocationAdapter, OrganizationAdapter},
		clinvoice_data::{chrono::Utc, Decimal, Money, views::LocationView},
		std::time::Instant,
	};

	#[test]
	fn test_into_view()
	{
		let start = Instant::now();

		util::test_temp_store(|store|
		{
			let earth = BincodeLocation::create(
				"Earth".into(),
				*store,
			).unwrap();

			let big_test = BincodeOrganization::create(
				earth.location.clone(),
				"Big Old Test Corporation".into(),
				*store,
			).unwrap();

			let create_job = BincodeJob::create(
				big_test.organization.clone(),
				Utc::now(),
				Money::new(Decimal::new(200, 2), ""),
				"Test the job creation function.",
				*store,
			).unwrap();

			let create_job_view = JobView
			{
				client: OrganizationView
				{
					location: LocationView
					{
						name: earth.location.name,
						outer: None,
					},
					name: big_test.organization.name,
				},
				date_close: create_job.job.date_close,
				date_open: create_job.job.date_open,
				invoice: create_job.job.invoice.clone(),
				notes: create_job.job.notes.clone(),
				objectives: create_job.job.objectives.clone(),
				timesheets: create_job.job.timesheets.clone(),
			};

			let create_job_view_result: DynamicResult<JobView> = create_job.into();

			assert_eq!(create_job_view, create_job_view_result.unwrap());

			println!("\n>>>>> BincodeJob test_into_view {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}
}

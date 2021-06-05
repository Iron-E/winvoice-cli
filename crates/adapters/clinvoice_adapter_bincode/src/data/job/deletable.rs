use
{
	std::{fs, io::ErrorKind},

	super::BincodeJob,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Deletable,
};

impl Deletable for BincodeJob<'_, '_>
{
	type Error = Error;

	fn delete(&self, _cascade: bool) -> Result<()>
	{
		if let Err(e) = fs::remove_file(self.filepath())
		{
			// We don't care if a file is missing; we want it deleted anyway.
			if e.kind() != ErrorKind::NotFound
			{
				return Err(e.into());
			}
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		std::time::Instant,

		super::{BincodeJob, Deletable},
		crate::{data::BincodeOrganization, util},

		clinvoice_adapter::data::{JobAdapter, OrganizationAdapter},
		clinvoice_data::
		{
			chrono::Utc,
			finance::{Currency, Money},
			Id, Location,
		},
	};

	#[test]
	fn delete()
	{
		util::temp_store(|store|
		{
			let big_test = BincodeOrganization
			{
				organization: &BincodeOrganization::create(
					Location {id: Id::new_v4(), name: "".into(), outer_id: None},
					"Big Old Test Corporation".into(),
					&store,
				).unwrap(),
				store,
			};

			let create_job = BincodeJob
			{
				job: &BincodeJob::create(
					big_test.organization.clone(),
					Utc::now(),
					Money::new(2_00, 2, Currency::USD),
					"Test the job creation function".into(),
					&store,
				).unwrap(),
				store,
			};

			let assert_job = BincodeJob
			{
				job: &BincodeJob::create(
					big_test.organization.clone(),
					Utc::now(),
					Money::new(2_00, 2, Currency::USD),
					"Assert that this stuff works".into(),
					&store,
				).unwrap(),
				store,
			};

			let start = Instant::now();
			// Delete both jobs
			create_job.delete(true).unwrap();
			assert_job.delete(true).unwrap();
			println!("\n>>>>> BincodeJob::delete {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 2);

			// Assert that all jobs are gone but the organization exists
			assert!(!&assert_job.filepath().is_file());
			assert!(&big_test.filepath().is_file());
			assert!(!&create_job.filepath().is_file());
		});
	}
}

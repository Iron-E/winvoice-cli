use
{
	super::BincodeJob,
	clinvoice_adapter::data::Deletable,
	std::{error::Error, fs, io::ErrorKind},
};

impl Deletable for BincodeJob<'_, '_, '_>
{
	fn delete(&self, _cascade: bool) -> Result<(), Box<dyn Error>>
	{
		if let Err(e) = fs::remove_file(self.filepath())
		{
			// We don't care if a file is missing; we want it deleted anyway.
			if e.kind() != ErrorKind::NotFound
			{
				return Err(e.into());
			}
		}

		return Ok(());
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeJob, Deletable},
		crate::{data::BincodeOrganization, util},
		clinvoice_adapter::data::{JobAdapter, OrganizationAdapter},
		clinvoice_data::{chrono::Utc, Decimal, Id, Location, Money},
		std::{collections::HashSet, time::Instant},
	};

	#[test]
	fn test_delete()
	{
		let start = Instant::now();

		util::test_temp_store(|store|
		{
			let big_test = BincodeOrganization::create(
				Location {id: Id::new_v4(), name: "".into(), outer_id: None},
				"Big Old Test Corporation".into(),
				HashSet::new(),
				*store,
			).unwrap();

			let create_job = BincodeJob::create(
				big_test.organization.clone(),
				Utc::now(),
				Money::new(Decimal::new(200, 2), ""),
				"Test the job creation function.",
				*store,
			).unwrap();

			let assert_job = BincodeJob::create(
				big_test.organization.clone(),
				Utc::now(),
				Money::new(Decimal::new(200, 2), "USD"),
				"Assert that this stuff works.",
				*store,
			).unwrap();

			// Delete both jobs
			create_job.delete(true).unwrap();
			assert_job.delete(true).unwrap();

			// Assert that all jobs are gone but the organization exists
			assert!(!&assert_job.filepath().is_file());
			assert!(&big_test.filepath().is_file());
			assert!(!&create_job.filepath().is_file());

			println!("\n>>>>> BincodeJob test_delete {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}
}

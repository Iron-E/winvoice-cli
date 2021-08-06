use std::io::ErrorKind;

use clinvoice_adapter::data::Deletable;
use tokio::fs;

use super::BincodeJob;
use crate::data::{Error, Result};

#[async_trait::async_trait]
impl Deletable for BincodeJob<'_, '_>
{
	type Error = Error;

	async fn delete(&self, _cascade: bool) -> Result<()>
	{
		if let Err(e) = fs::remove_file(self.filepath()).await
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
	use std::time::Instant;

	use clinvoice_adapter::data::{JobAdapter, OrganizationAdapter};
	use clinvoice_data::{
		chrono::Utc,
		finance::{Currency, Money},
		Id,
		Location,
	};

	use super::{BincodeJob, Deletable};
	use crate::{data::BincodeOrganization, util};

	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn delete()
	{
		let store = util::temp_store();

		let big_test = BincodeOrganization {
			organization: &BincodeOrganization::create(
				Location {
					id: Id::new_v4(),
					name: "".into(),
					outer_id: None,
				},
				"Big Old Test Corporation".into(),
				&store,
			)
			.await
			.unwrap(),
			store: &store,
		};

		let create_job = BincodeJob {
			job:   &BincodeJob::create(
				big_test.organization.clone(),
				Utc::now(),
				Money::new(2_00, 2, Currency::USD),
				"Test the job creation function".into(),
				&store,
			)
			.await
			.unwrap(),
			store: &store,
		};

		let assert_job = BincodeJob {
			job:   &BincodeJob::create(
				big_test.organization.clone(),
				Utc::now(),
				Money::new(2_00, 2, Currency::USD),
				"Assert that this stuff works".into(),
				&store,
			)
			.await
			.unwrap(),
			store: &store,
		};

		let start = Instant::now();
		// Delete both jobs
		create_job.delete(true).await.unwrap();
		assert_job.delete(true).await.unwrap();
		println!(
			"\n>>>>> BincodeJob::delete {}us <<<<<\n",
			Instant::now().duration_since(start).as_micros() / 2
		);

		// Assert that all jobs are gone but the organization exists
		assert!(!&assert_job.filepath().is_file());
		assert!(&big_test.filepath().is_file());
		assert!(!&create_job.filepath().is_file());
	}
}

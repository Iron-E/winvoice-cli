use
{
	std::{borrow::Cow::Borrowed, io::ErrorKind},

	super::BincodeEmployee,
	crate::data::{BincodeJob, Error, Result},

	clinvoice_adapter::data::{Deletable, Error as DataError, JobAdapter, Updatable},
	clinvoice_query as query,

	futures::stream::{self as stream, TryStreamExt},
	tokio::fs,
};

#[async_trait::async_trait]
impl Deletable for BincodeEmployee<'_, '_>
{
	type Error = Error;

	async fn delete(&self, cascade: bool) -> Result<()>
	{
		let associated_jobs = BincodeJob::retrieve(
			&query::Job
			{
				timesheets: query::Timesheet
				{
					employee: query::Employee
					{
						id: query::Match::HasAny(vec![Borrowed(&self.employee.id)].into_iter().collect()),
						..Default::default()
					},
					..Default::default()
				},
				..Default::default()
			},
			self.store,
		).await?;

		if cascade
		{
			stream::iter(associated_jobs.into_iter().map(Ok)).try_for_each_concurrent(None,
				|mut result| async move
				{
					result.timesheets = result.timesheets.into_iter()
						.filter(|t| t.employee_id != self.employee.id)
						.collect()
					;

					BincodeJob {job: &result, store: self.store}.update().await
				}
			).await?;
		}
		else if !associated_jobs.is_empty()
		{
			return Err(DataError::DeleteRestricted(self.employee.id).into());
		}

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
	use
	{
		std::time::Instant,

		super::{BincodeEmployee, BincodeJob, Borrowed, Deletable, JobAdapter, query, Updatable},
		crate::
		{
			data::{BincodeLocation, BincodeOrganization, BincodePerson},
			util,
		},

		clinvoice_adapter::data::{EmployeeAdapter, LocationAdapter, OrganizationAdapter, PersonAdapter},
		clinvoice_data::
		{
			chrono::Utc,
			finance::{Currency, Money},
			Contact, EmployeeStatus,
		},
	};

	#[tokio::test]
	async fn delete()
	{
		let store = util::temp_store();

		let earth = BincodeLocation
		{
			location: &BincodeLocation::create("Earth".into(), &store).await.unwrap(),
			store: &store,
		};

		let mut big_old_test = BincodeOrganization::create(
			earth.location.clone(),
			"Big Old Test Corporation".into(),
			&store,
		).await.unwrap();

		let testy = BincodePerson
		{
			person: &BincodePerson::create(
				"Testy Mćtesterson".into(),
				&store,
			).await.unwrap(),
			store: &store,
		};

		let ceo_testy = BincodeEmployee
		{
			employee: &BincodeEmployee::create(
				vec![("Work".into(), Contact::Address {location_id: earth.location.id, export: false})].into_iter().collect(),
				big_old_test.clone(),
				testy.person.clone(),
				EmployeeStatus::Employed,
				"CEO of Tests".into(),
				&store,
			).await.unwrap(),
			store: &store,
		};

		let mut creation = BincodeJob::create(
			big_old_test.clone(),
			Utc::now(),
			Money::new(2_00, 2, Currency::USD),
			"Test the job creation function".into(),
			&store,
		).await.unwrap();

		creation.start_timesheet(ceo_testy.employee.id);
		BincodeJob {job: &creation, store: &store}.update().await.unwrap();

		let start = Instant::now();
		// Assert that the deletion fails when restricted
		assert!(ceo_testy.delete(false).await.is_err());
		// Assert that the deletion works when cascading
		assert!(ceo_testy.delete(true).await.is_ok());
		println!("\n>>>>> BincodeEmployee::delete {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 2);

		// Assert the deleted file is gone.
		assert!(!ceo_testy.filepath().is_file());

		// Assert that the relevant files still exist
		assert!(BincodeOrganization {organization: &big_old_test, store: &store}.filepath().is_file());
		assert!(BincodeJob {job: &creation, store: &store}.filepath().is_file());
		assert!(earth.filepath().is_file());
		assert!(testy.filepath().is_file());

		// NOTE: I don't know if this statement is really necessary.
		big_old_test = BincodeOrganization::retrieve(
			&query::Organization
			{
				id: query::Match::EqualTo(Borrowed(&big_old_test.id)),
				..Default::default()
			},
			&store,
		).await.unwrap().iter().next().unwrap().clone();

		creation = BincodeJob::retrieve(
			&query::Job
			{
				client: query::Organization
				{
					id: query::Match::EqualTo(Borrowed(&big_old_test.id)),
					..Default::default()
				},
				id: query::Match::EqualTo(Borrowed(&creation.id)),
				..Default::default()
			},
			&store,
		).await.unwrap().iter().next().unwrap().clone();

		// Assert that no references to the deleted entity remain.
		assert!(creation.timesheets.iter().all(|t| t.employee_id != ceo_testy.employee.id));
	}
}

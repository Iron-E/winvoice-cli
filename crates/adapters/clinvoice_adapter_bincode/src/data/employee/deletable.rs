use
{
	super::BincodeEmployee,
	crate::data::{BincodeJob, Error, Result},
	clinvoice_adapter::data::{Deletable, JobAdapter, Match, query, Updatable},
	std::{borrow::Cow, fs, io::ErrorKind},
};

impl Deletable for BincodeEmployee<'_, '_>
{
	type Error = Error;

	fn delete(&self, cascade: bool) -> Result<()>
	{
		if let Err(e) = fs::remove_file(self.filepath())
		{
			// We don't care if a file is missing; we want it deleted anyway.
			if e.kind() != ErrorKind::NotFound
			{
				return Err(e.into());
			}
		}

		if cascade
		{
			for mut result in BincodeJob::retrieve(
				query::Job
				{
					timesheets: query::Timesheet
					{
						employee: query::Employee
						{
							id: Match::HasAny(vec![Cow::Borrowed(&self.employee.id)].into_iter().collect()),
							..Default::default()
						},
						..Default::default()
					},
					..Default::default()
				},
				self.store,
			)?
			{
				result.timesheets = result.timesheets.into_iter()
					.filter(|t| t.employee_id != self.employee.id)
					.collect()
				;

				BincodeJob {job: &result, store: self.store}.update()?;
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
		super::{BincodeEmployee, BincodeJob, Cow, Deletable, JobAdapter, Match, query, Updatable},
		crate::
		{
			data::{BincodeLocation, BincodeOrganization, BincodePerson},
			util,
		},
		clinvoice_adapter::data::{EmployeeAdapter, LocationAdapter, OrganizationAdapter, PersonAdapter},
		clinvoice_data::{chrono::Utc, Contact, Decimal, EmployeeStatus, Money},
		std::time::Instant,
	};

	#[test]
	fn delete()
	{
		util::temp_store(|store|
		{
			let earth = BincodeLocation
			{
				location: &BincodeLocation::create("Earth", &store).unwrap(),
				store,
			};

			let mut big_old_test = BincodeOrganization::create(
				earth.location.clone(),
				"Big Old Test Corporation",
				&store,
			).unwrap();

			let testy = BincodePerson
			{
				person: &BincodePerson::create(
					"Testy Mćtesterson",
					&store,
				).unwrap(),
				store,
			};

			let ceo_testy = BincodeEmployee
			{
				employee: &BincodeEmployee::create(
					vec![("Work".into(), Contact::Address(earth.location.id))].into_iter().collect(),
					big_old_test.clone(),
					testy.person.clone(),
					EmployeeStatus::Employed,
					"CEO of Tests",
					&store,
				).unwrap(),
				store,
			};

			let mut creation = BincodeJob::create(
				big_old_test.clone(),
				Utc::now(),
				Money::new(Decimal::new(200, 2), "USD"),
				"Test the job creation function",
				&store,
			).unwrap();

			creation.start_timesheet(ceo_testy.employee.id);
			BincodeJob {job: &creation, store}.update().unwrap();

			let start = Instant::now();
			// Assert that the deletion works
			assert!(ceo_testy.delete(true).is_ok());
			println!("\n>>>>> BincodeEmployee::delete {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

			// Assert the deleted file is gone.
			assert!(!ceo_testy.filepath().is_file());

			// Assert that the relevant files still exist
			assert!(BincodeOrganization {organization: &big_old_test, store}.filepath().is_file());
			assert!(BincodeJob {job: &creation, store}.filepath().is_file());
			assert!(earth.filepath().is_file());
			assert!(testy.filepath().is_file());

			big_old_test = BincodeOrganization::retrieve(
				query::Organization
				{
					id: Match::EqualTo(Cow::Borrowed(&big_old_test.id)),
					..Default::default()
				},
				&store,
			).unwrap().iter().next().unwrap().clone();

			creation = BincodeJob::retrieve(
				query::Job
				{
					client: query::Organization
					{
						id: Match::EqualTo(Cow::Borrowed(&big_old_test.id)),
						..Default::default()
					},
					id: Match::EqualTo(Cow::Borrowed(&creation.id)),
					..Default::default()
				},
				&store,
			).unwrap().iter().next().unwrap().clone();

			// Assert that no references to the deleted entity remain.
			assert!(creation.timesheets.iter().all(|t| t.employee_id != ceo_testy.employee.id));
		});
	}
}

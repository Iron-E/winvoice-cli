use
{
	super::BincodeEmployee,
	crate::data::{BincodeJob, BincodeOrganization},
	clinvoice_adapter::data::{Deletable, JobAdapter, MatchWhen, OrganizationAdapter, Updatable},
	std::{error::Error, fs, io::ErrorKind},
};

impl Deletable for BincodeEmployee<'_, '_, '_>
{
	fn delete(&self, cascade: bool) -> Result<(), Box<dyn Error>>
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
				MatchWhen::Any, // client
				MatchWhen::Any, // date close
				MatchWhen::Any, // date open
				MatchWhen::Any, // id
				MatchWhen::Any, // invoice date issued
				MatchWhen::Any, // invoice date paid
				MatchWhen::Any, // invoice hourly rate
				MatchWhen::Any, // notes
				MatchWhen::Any, // objectives
				MatchWhen::HasAny([self.employee.id].iter().cloned().collect()), // timesheet employee
				MatchWhen::Any, // timesheet time begin
				MatchWhen::Any, // timesheet time end
				self.store,
			)?
			{
				result.job.timesheets = result.job.timesheets.into_iter().filter(|t| t.employee_id != self.employee.id).collect();
				result.update()?;
			}

			for mut result in BincodeOrganization::retrieve(
				MatchWhen::Any, // id
				MatchWhen::Any, // location
				MatchWhen::Any, // name
				MatchWhen::HasAll([self.employee.id].iter().cloned().collect()), // representatives
				self.store,
			)?
			{
				result.organization.representatives.remove(&self.employee.id);
				result.update()?;
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
		super::{BincodeEmployee, BincodeJob, BincodeOrganization, Deletable, JobAdapter, MatchWhen, OrganizationAdapter, Updatable},
		crate::
		{
			data::{BincodeLocation, BincodePerson},
			util,
		},
		clinvoice_adapter::data::{EmployeeAdapter, LocationAdapter, PersonAdapter},
		clinvoice_data::{chrono::Utc, Contact, Decimal, Money},
		std::{collections::HashSet, time::Instant},
	};

	#[test]
	fn test_delete()
	{
		let start = Instant::now();

		util::test_temp_store(|store|
		{
			let earth = BincodeLocation::create("Earth", *store).unwrap();

			let mut big_old_test = BincodeOrganization::create(
				earth.location.clone(),
				"Big Old Test Corporation",
				HashSet::new(),
				*store,
			).unwrap();

			let mut contact_info = HashSet::new();
			contact_info.insert(Contact::Address(earth.location.id));

			let testy = BincodePerson::create(
				contact_info.clone(),
				"Testy Mćtesterson",
				*store,
			).unwrap();

			let ceo_testy = BincodeEmployee::create(
				contact_info.clone(),
				big_old_test.organization.clone(),
				testy.person.clone(),
				"CEO of Tests",
				*store,
			).unwrap();

			let mut creation = BincodeJob::create(
				big_old_test.organization.clone(),
				Utc::now(),
				Money::new(Decimal::new(200, 2), "USD"),
				"Test the job creation function.",
				*store,
			).unwrap();

			creation.job.start_timesheet(ceo_testy.employee.id);
			creation.update().unwrap();

			// Assert that the deletion works
			assert!(ceo_testy.delete(true).is_ok());

			// Assert the deleted file is gone.
			assert!(!ceo_testy.filepath().is_file());

			// Assert that the relevant files still exist
			assert!(big_old_test.filepath().is_file());
			assert!(creation.filepath().is_file());
			assert!(earth.filepath().is_file());
			assert!(testy.filepath().is_file());

			big_old_test = BincodeOrganization::retrieve(
				MatchWhen::EqualTo(big_old_test.organization.id), // id
				MatchWhen::Any, // location
				MatchWhen::Any, // name
				MatchWhen::Any, // representatives
				*store,
			).unwrap().iter().next().unwrap().clone();

			creation = BincodeJob::retrieve(
				MatchWhen::EqualTo(big_old_test.organization.id), // client
				MatchWhen::Any, // date close
				MatchWhen::Any, // date open
				MatchWhen::EqualTo(creation.job.id), // id
				MatchWhen::Any, // invoice date issued
				MatchWhen::Any, // invoice date paid
				MatchWhen::Any, // invoice hourly rate
				MatchWhen::Any, // notes
				MatchWhen::Any, // objectives
				MatchWhen::Any, // timesheet employee
				MatchWhen::Any, // timesheet time begin
				MatchWhen::Any, // timesheet time end
				*store,
			).unwrap().iter().next().unwrap().clone();

			// Assert that no references to the deleted entity remain.
			assert!(big_old_test.organization.representatives.iter().all(|id| *id != ceo_testy.employee.id));
			assert!(creation.job.timesheets.iter().all(|t| t.employee_id != ceo_testy.employee.id));

			println!("\n>>>>> BincodeEmployee test_delete {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}
}

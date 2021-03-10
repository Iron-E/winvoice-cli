use
{
	super::BincodeOrganization,
	crate::data::{BincodeEmployee, BincodeJob, Error, Result},
	clinvoice_adapter::data::{Deletable, EmployeeAdapter, JobAdapter, MatchWhen},
	std::{fs, io::ErrorKind},
};

impl Deletable for BincodeOrganization<'_, '_>
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
			BincodeJob::retrieve(
				MatchWhen::EqualTo(self.organization.id), // client
				MatchWhen::Any, // date close
				MatchWhen::Any, // date open
				MatchWhen::Any, // id
				MatchWhen::Any, // invoice date
				MatchWhen::Any, // invoice hourly rate
				MatchWhen::Any, // notes
				MatchWhen::Any, // objectives
				MatchWhen::Any, // timesheet employee
				MatchWhen::Any, // timesheet time begin
				MatchWhen::Any, // timesheet time end
				self.store,
			)?.into_iter().try_for_each(|j|
				BincodeJob
				{
					job: &j,
					store: self.store,
				}.delete(true)
			)?;

			BincodeEmployee::retrieve(
				MatchWhen::Any, // contact info
				MatchWhen::Any, // id
				MatchWhen::EqualTo(self.organization.id), // organization
				MatchWhen::Any, // person
				MatchWhen::Any, // title
				MatchWhen::Any, // status
				self.store,
			)?.into_iter().try_for_each(|e|
				BincodeEmployee
				{
					employee: &e,
					store: self.store,
				}.delete(true)
			)?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeEmployee, BincodeJob, BincodeOrganization, Deletable, JobAdapter},
		crate::
		{
			data::{BincodeLocation, BincodePerson},
			util,
		},
		clinvoice_adapter::data::{EmployeeAdapter, LocationAdapter, OrganizationAdapter, PersonAdapter, Updatable},
		clinvoice_data::{chrono::Utc, Contact, Decimal, EmployeeStatus, Money},
		std::time::Instant,
	};

	#[test]
	fn test_delete()
	{
		util::test_temp_store(|store|
		{
			let earth = BincodeLocation
			{
				location: &BincodeLocation::create("Earth", &store).unwrap(),
				store,
			};

			let big_old_test = BincodeOrganization
			{
				organization: &BincodeOrganization::create(
					earth.location.clone(),
					"Big Old Test Corporation",
					&store,
				).unwrap(),
				store,
			};

			let mut contact_info = Vec::new();
			contact_info.push(Contact::Address(earth.location.id));

			let testy = BincodePerson
			{
				person: &BincodePerson::create(
					contact_info.clone(),
					"Testy Mćtesterson",
					&store,
				).unwrap(),
				store,
			};

			let ceo_testy = BincodeEmployee
			{
				employee: &BincodeEmployee::create(
					contact_info.clone(),
					big_old_test.organization.clone(),
					testy.person.clone(),
					EmployeeStatus::Representative,
					"CEO of Tests",
					&store,
				).unwrap(),
				store,
			};

			let mut creation = BincodeJob::create(
				big_old_test.organization.clone(),
				Utc::now(),
				Money::new(Decimal::new(200, 2), "USD"),
				"Test the job creation function.",
				&store,
			).unwrap();

			creation.start_timesheet(ceo_testy.employee.id);
			BincodeJob {job: &creation, store}.update().unwrap();

			let start = Instant::now();
			// Assert that the deletion works
			assert!(big_old_test.delete(true).is_ok());
			println!("\n>>>>> BincodeOrganization::delete {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

			// Assert that the dependent files are gone
			assert!(!big_old_test.filepath().is_file());
			assert!(!ceo_testy.filepath().is_file());
			assert!(!BincodeJob {job: &creation, store}.filepath().is_file());

			// Assert that the independent files are present
			assert!(earth.filepath().is_file());
			assert!(testy.filepath().is_file());
		});
	}
}

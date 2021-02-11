use
{
	super::BincodeOrganization,
	crate::data::{BincodeEmployee, BincodeJob},
	clinvoice_adapter::data::{Deletable, EmployeeAdapter, JobAdapter, MatchWhen},
	std::{error::Error, fs, io::ErrorKind},
};

impl Deletable for BincodeOrganization<'_, '_, '_>
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
			for result in BincodeJob::retrieve(
				MatchWhen::EqualTo(self.organization.id), // client
				MatchWhen::Any, // date close
				MatchWhen::Any, // date open
				MatchWhen::Any, // id
				MatchWhen::Any, // invoice date issued
				MatchWhen::Any, // invoice date paid
				MatchWhen::Any, // invoice hourly rate
				MatchWhen::Any, // notes
				MatchWhen::Any, // objectives
				MatchWhen::Any, // timesheet employee
				MatchWhen::Any, // timesheet time begin
				MatchWhen::Any, // timesheet time end
				self.store,
			)? { result.delete(true)?; }

			for result in BincodeEmployee::retrieve(
				MatchWhen::Any, // contact info
				MatchWhen::Any, // employed
				MatchWhen::Any, // id
				MatchWhen::EqualTo(self.organization.id), // organization
				MatchWhen::Any, // person
				MatchWhen::Any, // title
				self.store,
			)? { result.delete(true)?; }
		}

		return Ok(());
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

			let big_old_test = BincodeOrganization::create(
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
			assert!(big_old_test.delete(true).is_ok());

			// Assert that the dependent files are gone
			assert!(!big_old_test.filepath().is_file());
			assert!(!ceo_testy.filepath().is_file());
			assert!(!creation.filepath().is_file());

			// Assert that the independent files are present
			assert!(earth.filepath().is_file());
			assert!(testy.filepath().is_file());

			println!("\n>>>>> BincodeEmployee test_delete {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}
}

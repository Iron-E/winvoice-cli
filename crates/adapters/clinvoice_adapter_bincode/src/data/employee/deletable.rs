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
		super::BincodeEmployee,
		crate::
		{
			data::{BincodeLocation, BincodeOrganization, BincodePerson},
			util
		},
		clinvoice_adapter::data::{Deletable, EmployeeAdapter, LocationAdapter, OrganizationAdapter, PersonAdapter},
		clinvoice_data::{Contact, Id},
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
				earth.location,
				"Big Old Test Corporation",
				HashSet::new(),
				*store,
			).unwrap();

			let mut contact_info = HashSet::new();
			contact_info.insert(Contact::Address(Id::new_v4()));

			let testy = BincodePerson::create(
				contact_info.clone(),
				"Testy Mćtesterson",
				*store,
			).unwrap();

			let ceo_testy = BincodeEmployee::create(
				contact_info.clone(),
				big_old_test.organization,
				testy.person,
				"CEO of Tests",
				*store,
			).unwrap();

			assert!(ceo_testy.delete(true).is_ok());

			// TODO: add assertions for whether or not the created jobs and orgs exist.

			println!("\n>>>>> BincodeEmployee test_delete {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}
}

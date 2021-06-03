use
{
	std::{borrow::Cow::Borrowed, fs, io::ErrorKind},

	super::BincodeOrganization,
	crate::data::{BincodeEmployee, BincodeJob, Error, Result},

	clinvoice_adapter::data::{Deletable, EmployeeAdapter, Error as DataError, JobAdapter},
	clinvoice_data::Employee,
	clinvoice_query as query,
};

impl Deletable for BincodeOrganization<'_, '_>
{
	type Error = Error;

	fn delete(&self, cascade: bool) -> Result<()>
	{
		let associated_employees = || -> Result<Vec<Employee>>
		{
			BincodeEmployee::retrieve(
				&query::Employee
				{
					organization: query::Organization
					{
						id: query::Match::EqualTo(Borrowed(&self.organization.id)),
						..Default::default()
					},
					..Default::default()
				},
				self.store,
			)
		};

		let associated_jobs = BincodeJob::retrieve(
			&query::Job
			{
				client: query::Organization
				{
					id: query::Match::EqualTo(Borrowed(&self.organization.id)),
					..Default::default()
				},
				..Default::default()
			},
			self.store,
		)?;

		if cascade
		{
			associated_jobs.into_iter().try_for_each(
				|j| BincodeJob {job: &j, store: self.store}.delete(cascade)
			)?;

			let associated_employees = associated_employees()?;
			associated_employees.into_iter().try_for_each(
				|e| BincodeEmployee {employee: &e, store: self.store}.delete(cascade)
			)?;
		}
		else if !(associated_jobs.is_empty() && associated_employees()?.is_empty())
		{
			return Err(DataError::DeleteRestricted(self.organization.id).into());
		}

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

		super::{BincodeEmployee, BincodeJob, BincodeOrganization, Deletable, JobAdapter},
		crate::
		{
			data::{BincodeLocation, BincodePerson},
			util,
		},

		clinvoice_adapter::data::{EmployeeAdapter, LocationAdapter, OrganizationAdapter, PersonAdapter, Updatable},
		clinvoice_data::
		{
			chrono::Utc,
			finance::{Currency, Money},
			Contact, EmployeeStatus,
		},
	};

	#[test]
	fn delete()
	{
		util::temp_store(|store|
		{
			let earth = BincodeLocation
			{
				location: &BincodeLocation::create("Earth".into(), &store).unwrap(),
				store,
			};

			let big_old_test = BincodeOrganization
			{
				organization: &BincodeOrganization::create(
					earth.location.clone(),
					"Big Old Test Corporation".into(),
					&store,
				).unwrap(),
				store,
			};

			let testy = BincodePerson
			{
				person: &BincodePerson::create(
					"Testy McTesterson".into(),
					&store,
				).unwrap(),
				store,
			};

			let ceo_testy = BincodeEmployee
			{
				employee: &BincodeEmployee::create(
					vec![("Work Address".into(), Contact::Address {location_id: earth.location.id, export: false})].into_iter().collect(),
					big_old_test.organization.clone(),
					testy.person.clone(),
					EmployeeStatus::Representative,
					"CEO of Tests".into(),
					&store,
				).unwrap(),
				store,
			};

			let mut creation = BincodeJob::create(
				big_old_test.organization.clone(),
				Utc::now(),
				Money::new(200, 2, Currency::USD),
				"Test the job creation function".into(),
				&store,
			).unwrap();

			creation.start_timesheet(ceo_testy.employee.id);
			BincodeJob {job: &creation, store}.update().unwrap();

			let start = Instant::now();
			// Assert that the deletion fails with restriction
			assert!(big_old_test.delete(false).is_err());
			// Assert that the deletion works when cascading
			assert!(big_old_test.delete(true).is_ok());
			println!("\n>>>>> BincodeOrganization::delete {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 2);

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

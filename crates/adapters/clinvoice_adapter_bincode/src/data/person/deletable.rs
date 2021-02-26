use
{
	super::BincodePerson,
	crate::data::BincodeEmployee,
	clinvoice_adapter::
	{
		DynamicResult,
		data::{Deletable, EmployeeAdapter, MatchWhen},
	},
	std::{fs, io::ErrorKind},
};

impl Deletable for BincodePerson<'_, '_, '_>
{
	fn delete(&self, cascade: bool) -> DynamicResult<()>
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
			for result in BincodeEmployee::retrieve(
				MatchWhen::Any, // contact info
				MatchWhen::Any, // id
				MatchWhen::Any, // organization
				MatchWhen::EqualTo(self.person.id), // person
				MatchWhen::Any, // title
				MatchWhen::Any, // status
				self.store,
			)? { result.delete(true)?; }
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeEmployee, BincodePerson, Deletable, EmployeeAdapter},
		crate::
		{
			data::{BincodeLocation, BincodeOrganization},
			util,
		},
		clinvoice_adapter::data::{LocationAdapter, OrganizationAdapter, PersonAdapter},
		clinvoice_data::{Contact, EmployeeStatus},
		std::time::Instant,
	};

	#[test]
	fn test_delete()
	{
		util::test_temp_store(|store|
		{
			let earth = BincodeLocation::create("Earth", *store).unwrap();

			let big_old_test = BincodeOrganization::create(
				earth.location.clone(),
				"Big Old Test Corporation",
				*store,
			).unwrap();

			let mut contact_info = Vec::new();
			contact_info.push(Contact::Address(earth.location.id));

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
				EmployeeStatus::Employed,
				*store,
			).unwrap();

			let start = Instant::now();
			// Assert that the deletion works
			assert!(testy.delete(true).is_ok());
			println!("\n>>>>> BincodePerson::delete {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

			// Assert that `testy` and its referencing employee is gone.
			assert!(!testy.filepath().is_file());
			assert!(!ceo_testy.filepath().is_file());

			// Assert that the independent files still exist.
			assert!(big_old_test.filepath().is_file());
			assert!(earth.filepath().is_file());
		});
	}
}

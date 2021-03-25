use
{
	super::BincodePerson,
	crate::data::{BincodeEmployee, Error, Result},
	clinvoice_adapter::data::{Deletable, EmployeeAdapter, MatchWhen},
	std::{borrow::Cow, fs, io::ErrorKind},
};

impl Deletable for BincodePerson<'_, '_>
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
			BincodeEmployee::retrieve(
				MatchWhen::Any, // contact info
				MatchWhen::Any, // id
				MatchWhen::Any, // organization
				MatchWhen::EqualTo(Cow::Borrowed(&self.person.id)), // person
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
					vec![("Office".into(), Contact::Address(earth.location.id))].into_iter().collect(),
					big_old_test.organization.clone(),
					testy.person.clone(),
					EmployeeStatus::Employed,
					"CEO of Tests",
					&store,
				).unwrap(),
				store,
			};

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

use
{
	std::{borrow::Cow, fs, io::ErrorKind},

	super::BincodePerson,
	crate::data::{BincodeEmployee, Error, Result},

	clinvoice_adapter::data::{Deletable, EmployeeAdapter, Error as DataError, Match, query},
};

impl Deletable for BincodePerson<'_, '_>
{
	type Error = Error;

	fn delete(&self, cascade: bool) -> Result<()>
	{
		let associated_employees = BincodeEmployee::retrieve(
			query::Employee
			{
				person: query::Person
				{
					id: Match::EqualTo(Cow::Borrowed(&self.person.id)),
					..Default::default()
				},
				..Default::default()
			},
			self.store,
		)?;

		if cascade
		{
			associated_employees.into_iter().try_for_each(
				|e| BincodeEmployee {employee: &e, store: self.store}.delete(true)
			)?;
		}
		else if !associated_employees.is_empty()
		{
			return Err(DataError::DeleteRestricted(self.person.id).into());
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

		super::{BincodeEmployee, BincodePerson, Deletable, EmployeeAdapter},
		crate::
		{
			data::{BincodeLocation, BincodeOrganization},
			util,
		},

		clinvoice_adapter::data::{LocationAdapter, OrganizationAdapter, PersonAdapter},
		clinvoice_data::{Contact, EmployeeStatus},
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
			// Assert that the deletion fails when restricted
			assert!(testy.delete(false).is_err());
			// Assert that the deletion works when cascading
			assert!(testy.delete(true).is_ok());
			println!("\n>>>>> BincodePerson::delete {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 2);

			// Assert that `testy` and its referencing employee is gone.
			assert!(!testy.filepath().is_file());
			assert!(!ceo_testy.filepath().is_file());

			// Assert that the independent files still exist.
			assert!(big_old_test.filepath().is_file());
			assert!(earth.filepath().is_file());
		});
	}
}

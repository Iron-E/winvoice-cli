use std::{
	borrow::Cow::Borrowed,
	fs,
	io::ErrorKind,
};

use clinvoice_adapter::data::{
	Deletable,
	Error as DataError,
	JobAdapter,
	Updatable,
};
use clinvoice_query as query;

use super::BincodeEmployee;
use crate::data::{
	BincodeJob,
	Error,
	Result,
};

impl Deletable for BincodeEmployee<'_, '_>
{
	type Error = Error;

	fn delete(&self, cascade: bool) -> Result<()>
	{
		let associated_jobs = BincodeJob::retrieve(
			&query::Job {
				timesheets: query::Timesheet {
					employee: query::Employee {
						id: query::Match::HasAny(vec![Borrowed(&self.employee.id)].into_iter().collect()),
						..Default::default()
					},
					..Default::default()
				},
				..Default::default()
			},
			self.store,
		)?;

		if cascade
		{
			associated_jobs.into_iter().try_for_each(|mut result| {
				result.timesheets = result
					.timesheets
					.into_iter()
					.filter(|t| t.employee_id != self.employee.id)
					.collect();

				BincodeJob {
					job:   &result,
					store: self.store,
				}
				.update()
			})?;
		}
		else if !associated_jobs.is_empty()
		{
			return Err(DataError::DeleteRestricted(self.employee.id).into());
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
	use std::time::Instant;

	use clinvoice_adapter::data::{
		EmployeeAdapter,
		LocationAdapter,
		OrganizationAdapter,
		PersonAdapter,
	};
	use clinvoice_data::{
		chrono::Utc,
		finance::{
			Currency,
			Money,
		},
		Contact,
		EmployeeStatus,
	};

	use super::{
		query,
		BincodeEmployee,
		BincodeJob,
		Borrowed,
		Deletable,
		JobAdapter,
		Updatable,
	};
	use crate::{
		data::{
			BincodeLocation,
			BincodeOrganization,
			BincodePerson,
		},
		util,
	};

	#[test]
	fn delete()
	{
		util::temp_store(|store| {
			let earth = BincodeLocation {
				location: &BincodeLocation::create("Earth".into(), &store).unwrap(),
				store,
			};

			let mut big_old_test = BincodeOrganization::create(
				earth.location.clone(),
				"Big Old Test Corporation".into(),
				&store,
			)
			.unwrap();

			let testy = BincodePerson {
				person: &BincodePerson::create("Testy Mćtesterson".into(), &store).unwrap(),
				store,
			};

			let ceo_testy = BincodeEmployee {
				employee: &BincodeEmployee::create(
					vec![("Work".into(), Contact::Address {
						location_id: earth.location.id,
						export:      false,
					})]
					.into_iter()
					.collect(),
					big_old_test.clone(),
					testy.person.clone(),
					EmployeeStatus::Employed,
					"CEO of Tests".into(),
					&store,
				)
				.unwrap(),
				store,
			};

			let mut creation = BincodeJob::create(
				big_old_test.clone(),
				Utc::now(),
				Money::new(2_00, 2, Currency::USD),
				"Test the job creation function".into(),
				&store,
			)
			.unwrap();

			creation.start_timesheet(ceo_testy.employee.id);
			BincodeJob {
				job: &creation,
				store,
			}
			.update()
			.unwrap();

			let start = Instant::now();
			// Assert that the deletion fails when restricted
			assert!(ceo_testy.delete(false).is_err());
			// Assert that the deletion works when cascading
			assert!(ceo_testy.delete(true).is_ok());
			println!(
				"\n>>>>> BincodeEmployee::delete {}us <<<<<\n",
				Instant::now().duration_since(start).as_micros() / 2
			);

			// Assert the deleted file is gone.
			assert!(!ceo_testy.filepath().is_file());

			// Assert that the relevant files still exist
			assert!(BincodeOrganization {
				organization: &big_old_test,
				store
			}
			.filepath()
			.is_file());
			assert!(BincodeJob {
				job: &creation,
				store
			}
			.filepath()
			.is_file());
			assert!(earth.filepath().is_file());
			assert!(testy.filepath().is_file());

			big_old_test = BincodeOrganization::retrieve(
				&query::Organization {
					id: query::Match::EqualTo(Borrowed(&big_old_test.id)),
					..Default::default()
				},
				&store,
			)
			.unwrap()
			.iter()
			.next()
			.unwrap()
			.clone();

			creation = BincodeJob::retrieve(
				&query::Job {
					client: query::Organization {
						id: query::Match::EqualTo(Borrowed(&big_old_test.id)),
						..Default::default()
					},
					id: query::Match::EqualTo(Borrowed(&creation.id)),
					..Default::default()
				},
				&store,
			)
			.unwrap()
			.iter()
			.next()
			.unwrap()
			.clone();

			// Assert that no references to the deleted entity remain.
			assert!(creation
				.timesheets
				.iter()
				.all(|t| t.employee_id != ceo_testy.employee.id));
		});
	}
}

use std::{borrow::Cow::Borrowed, io::ErrorKind};

use clinvoice_adapter::data::{Deletable, EmployeeAdapter, Error as DataError};
use clinvoice_query as query;
use futures::stream::{self, TryStreamExt};
use tokio::fs;

use super::BincodePerson;
use crate::data::{BincodeEmployee, Error, Result};

#[async_trait::async_trait]
impl Deletable for BincodePerson<'_, '_>
{
	type Error = Error;

	async fn delete(&self, cascade: bool) -> Result<()>
	{
		let associated_employees = BincodeEmployee::retrieve(
			&query::Employee {
				person: query::Person {
					id: query::Match::EqualTo(Borrowed(&self.person.id)),
					..Default::default()
				},
				..Default::default()
			},
			self.store,
		)
		.await?;

		if cascade
		{
			stream::iter(associated_employees.into_iter().map(Ok))
				.try_for_each_concurrent(None, |e| async move {
					BincodeEmployee {
						employee: &e,
						store:    self.store,
					}
					.delete(cascade)
					.await
				})
				.await?;
		}
		else if !associated_employees.is_empty()
		{
			return Err(DataError::DeleteRestricted(self.person.id).into());
		}

		if let Err(e) = fs::remove_file(self.filepath()).await
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

	use clinvoice_adapter::data::{LocationAdapter, OrganizationAdapter, PersonAdapter};
	use clinvoice_data::{Contact, EmployeeStatus};

	use super::{BincodeEmployee, BincodePerson, Deletable, EmployeeAdapter};
	use crate::{
		data::{BincodeLocation, BincodeOrganization},
		util,
	};

	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn delete()
	{
		let store = util::temp_store();

		let earth = BincodeLocation {
			location: &BincodeLocation::create("Earth".into(), &store)
				.await
				.unwrap(),
			store:    &store,
		};

		let big_old_test = BincodeOrganization {
			organization: &BincodeOrganization::create(
				earth.location.clone(),
				"Big Old Test Corporation".into(),
				&store,
			)
			.await
			.unwrap(),
			store: &store,
		};

		let testy = BincodePerson {
			person: &BincodePerson::create("Testy Mćtesterson".into(), &store)
				.await
				.unwrap(),
			store:  &store,
		};

		let ceo_testy = BincodeEmployee {
			employee: &BincodeEmployee::create(
				vec![("Office".into(), Contact::Address {
					location_id: earth.location.id,
					export:      false,
				})]
				.into_iter()
				.collect(),
				big_old_test.organization.clone(),
				testy.person.clone(),
				EmployeeStatus::Employed,
				"CEO of Tests".into(),
				&store,
			)
			.await
			.unwrap(),
			store:    &store,
		};

		let start = Instant::now();
		// Assert that the deletion fails when restricted
		assert!(testy.delete(false).await.is_err());
		// Assert that the deletion works when cascading
		assert!(testy.delete(true).await.is_ok());
		println!(
			"\n>>>>> BincodePerson::delete {}us <<<<<\n",
			Instant::now().duration_since(start).as_micros() / 2
		);

		// Assert that `testy` and its referencing employee is gone.
		assert!(!testy.filepath().is_file());
		assert!(!ceo_testy.filepath().is_file());

		// Assert that the independent files still exist.
		assert!(big_old_test.filepath().is_file());
		assert!(earth.filepath().is_file());
	}
}

use std::{
	borrow::Cow::Borrowed,
	io::ErrorKind,
};

use clinvoice_adapter::data::{
	Deletable,
	Error as DataError,
	LocationAdapter,
	OrganizationAdapter,
};
use clinvoice_query as query;
use futures::stream::{
	self,
	TryStreamExt,
};
use tokio::fs;

use super::BincodeLocation;
use crate::data::{
	BincodeOrganization,
	Error,
	Result,
};

#[async_trait::async_trait]
impl Deletable for BincodeLocation<'_, '_>
{
	type Error = Error;

	async fn delete(&self, cascade: bool) -> Result<()>
	{
		let location_query = query::Location {
			outer: query::OuterLocation::Some(
				query::Location {
					id: query::Match::EqualTo(Borrowed(&self.location.id)),
					..Default::default()
				}
				.into(),
			),
			..Default::default()
		};

		let organization_query = query::Organization {
			location: query::Location {
				id: query::Match::EqualTo(Borrowed(&self.location.id)),
				..Default::default()
			},
			..Default::default()
		};

		let (associated_locations, associated_organizations) = futures::try_join!(
			BincodeLocation::retrieve(&location_query, self.store),
			BincodeOrganization::retrieve(&organization_query, self.store),
		)?;

		if cascade
		{
			stream::iter(associated_organizations.into_iter().map(Ok))
				.try_for_each_concurrent(None, |o| async move {
					BincodeOrganization {
						organization: &o,
						store: self.store,
					}
					.delete(cascade)
					.await
				})
				.await?;

			stream::iter(associated_locations.into_iter().map(Ok))
				.try_for_each_concurrent(None, |l| async move {
					BincodeLocation {
						location: &l,
						store:    self.store,
					}
					.delete(cascade)
					.await
				})
				.await?;
		}
		else if !(associated_locations.is_empty() || associated_organizations.is_empty())
		{
			return Err(DataError::DeleteRestricted(self.location.id).into());
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

	use clinvoice_adapter::data::OrganizationAdapter;

	use super::{
		BincodeLocation,
		Deletable,
		LocationAdapter,
	};
	use crate::{
		data::BincodeOrganization,
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

		let usa = BincodeLocation {
			location: &earth.create_inner("USA".into()).await.unwrap(),
			store:    &store,
		};

		let arizona = BincodeLocation {
			location: &usa.create_inner("Arizona".into()).await.unwrap(),
			store:    &store,
		};

		let phoenix = BincodeLocation {
			location: &arizona.create_inner("Phoenix".into()).await.unwrap(),
			store:    &store,
		};

		let dogood = BincodeOrganization {
			organization: &BincodeOrganization::create(
				arizona.location.clone(),
				"DoGood Inc".into(),
				&store,
			)
			.await
			.unwrap(),
			store: &store,
		};

		let start = Instant::now();

		// delete just phoenix.
		phoenix.delete(false).await.unwrap();

		// assert that phoenix is gone.
		assert!(!phoenix.filepath().is_file());

		// Assert that every location inside the USA is there
		assert!(earth.filepath().is_file());
		assert!(usa.filepath().is_file());
		assert!(arizona.filepath().is_file());

		// assert that `dogood`, located in arizona, is there
		assert!(dogood.filepath().is_file());

		// delete the usa and everything in it.
		usa.delete(true).await.unwrap();

		println!(
			"\n>>>>> BincodeLocation::delete {}us <<<<<\n",
			Instant::now().duration_since(start).as_micros() / 2
		);

		// Assert that every location inside the USA is gone
		assert!(earth.filepath().is_file());
		assert!(!usa.filepath().is_file());
		assert!(!arizona.filepath().is_file());

		// assert that `dogood`, located in arizona, is gone.
		assert!(!dogood.filepath().is_file());
	}
}

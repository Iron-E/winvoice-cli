use
{
	std::{borrow::Cow::Borrowed, io::ErrorKind},

	super::BincodeLocation,
	crate::data::{BincodeOrganization, Error, Result},

	clinvoice_adapter::data::{Deletable, Error as DataError, LocationAdapter, OrganizationAdapter},
	clinvoice_query as query,

	futures::stream::{self, TryStreamExt},
	tokio::fs,
};

#[async_trait::async_trait]
impl Deletable for BincodeLocation<'_, '_>
{
	type Error = Error;

	async fn delete(&self, cascade: bool) -> Result<()>
	{
		let (associated_locations, associated_organizations) = futures::try_join!(
			BincodeLocation::retrieve(
				&query::Location
				{
					outer: query::OuterLocation::Some(
						query::Location
						{
							id: query::Match::EqualTo(Borrowed(&self.location.id)),
							..Default::default()
						}.into()
					),
					..Default::default()
				},
				self.store,
			),

			BincodeOrganization::retrieve(
				&query::Organization
				{
					location: query::Location
					{
						id: query::Match::EqualTo(Borrowed(&self.location.id)),
						..Default::default()
					},
					..Default::default()
				},
				self.store,
			),
		)?;

		if cascade
		{
			stream::iter(associated_organizations.into_iter().map(Ok)).try_for_each_concurrent(None,
				|o| BincodeOrganization {organization: &o, store: self.store}.delete(cascade)
			).await?;

			stream::iter(associated_locations.into_iter().map(Ok)).try_for_each_concurrent(None,
				|l| BincodeLocation {location: &l, store: self.store}.delete(cascade)
			).await?;
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
	use
	{
		std::time::Instant,

		super::{BincodeLocation, Deletable, LocationAdapter},
		crate::{data::BincodeOrganization, util},

		clinvoice_adapter::data::OrganizationAdapter,
	};

	#[tokio::test]
	async fn delete()
	{
		let store = util::temp_store();

		let earth = BincodeLocation
		{
			location: &BincodeLocation::create("Earth".into(), &store).await.unwrap(),
			store: &store,
		};

		let usa = BincodeLocation
		{
			location: &earth.create_inner("USA".into()).await.unwrap(),
			store: &store,
		};

		let arizona = BincodeLocation
		{
			location: &usa.create_inner("Arizona".into()).await.unwrap(),
			store: &store,
		};

		let phoenix = BincodeLocation
		{
			location: &arizona.create_inner("Phoenix".into()).await.unwrap(),
			store: &store,
		};

		let dogood = BincodeOrganization
		{
			organization: &BincodeOrganization::create(
				arizona.location.clone(),
				"DoGood Inc".into(),
				&store
			).await.unwrap(),
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

		println!("\n>>>>> BincodeLocation::delete {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 2);

		// Assert that every location inside the USA is gone
		assert!(earth.filepath().is_file());
		assert!(!usa.filepath().is_file());
		assert!(!arizona.filepath().is_file());

		// assert that `dogood`, located in arizona, is gone.
		assert!(!dogood.filepath().is_file());
	}
}

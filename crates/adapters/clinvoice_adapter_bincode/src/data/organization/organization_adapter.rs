use
{
	super::BincodeOrganization,
	crate::
	{
		data::{Error, Result},
		util,
	},

	clinvoice_adapter::
	{
		data::{Error as DataError, Initializable, OrganizationAdapter, Updatable},
		Store,
	},
	clinvoice_data::{Location, Organization},
	clinvoice_query as query,
};

#[async_trait::async_trait]
impl OrganizationAdapter for BincodeOrganization<'_, '_>
{
	type Error = Error;

	/// # Summary
	///
	/// Create a new [`Organization`] on the active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Organization`].
	///
	/// # Returns
	///
	/// The newly created [`Organization`].
	async fn create(location: Location, name: String, store: &Store) -> Result<Organization>
	{
		let init_fut = Self::init(&store);

		let organization = Organization
		{
			id: util::unique_id(&Self::path(&store))?,
			location_id: location.id,
			name,
		};

		init_fut.await?;
		BincodeOrganization {organization: &organization, store}.update().await?;

		Ok(organization)
	}

	/// # Summary
	///
	/// Retrieve some [`Organization`] from the active [`Store`]crate::Store).
	///
	/// # Parameters
	///
	/// See [`Organization`].
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`Job`]s.
	async fn retrieve(query: &query::Organization, store: &Store) -> Result<Vec<Organization>>
	{
		Self::init(&store).await?;

		util::retrieve(Self::path(store),
			|o| query.matches(o).map_err(|e| DataError::from(e).into())
		).await
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		std::{borrow::Cow::Borrowed, time::Instant},

		super::{BincodeOrganization, Location, Organization, OrganizationAdapter, query, Store, util},

		clinvoice_query::{Match, MatchStr},
		clinvoice_data::Id,

		tokio::fs,
	};

	#[tokio::test]
	async fn create()
	{
		let store = util::temp_store();

		let earth_id = Id::new_v4();
		let usa_id = Id::new_v4();
		let arizona_id = Id::new_v4();
		let phoenix_id = Id::new_v4();
		let some_id = Id::new_v4();

		let start = Instant::now();

		let (alsd, eal, aaa, focj, giguy) = futures::try_join!(
			BincodeOrganization::create(
				Location {name: "Earth".into(), id: Id::new_v4(), outer_id: None},
				"alsdkjaldkj".into(), &store
			),

			BincodeOrganization::create(
				Location {name: "USA".into(), id: usa_id, outer_id: Some(earth_id)},
				"alskdjalgkh  ladhkj EAL ISdh".into(), &store
			),

			BincodeOrganization::create(
				Location {name: "Arizona".into(), id: arizona_id, outer_id: Some(earth_id)},
				" AAA – 44 %%".into(), &store
			),

			BincodeOrganization::create(
				Location {name: "Phoenix".into(), id: phoenix_id, outer_id: Some(arizona_id)},
				" ^^^ ADSLKJDLASKJD FOCJCI".into(), &store
			),

			BincodeOrganization::create(
				Location {name: "Some Road".into(), id: some_id, outer_id: Some(phoenix_id)},
				"aldkj doiciuc giguy &&".into(), &store
			),
		).unwrap();

		println!("\n>>>>> BincodeOrganization::create {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 5);

		futures::join!(
			create_assertion(alsd, &store),
			create_assertion(eal, &store),
			create_assertion(aaa, &store),
			create_assertion(focj, &store),
			create_assertion(giguy, &store),
		);
	}

	async fn create_assertion(organization: Organization, store: &Store)
	{
		let read_result = fs::read(BincodeOrganization {organization: &organization, store}.filepath()).await.unwrap();
		assert_eq!(organization, bincode::deserialize(&read_result).unwrap());
	}

	#[tokio::test]
	async fn retrieve()
	{
		let store = util::temp_store();

		let earth_id = Id::new_v4();
		let usa_id = Id::new_v4();
		let arizona_id = Id::new_v4();

		let (packing, eal, aaa) =  futures::try_join!(
			BincodeOrganization::create(
				Location {name: "Earth".into(), id: earth_id, outer_id: None},
				"Packing Co".into(), &store
			),

			BincodeOrganization::create(
				Location {name: "USA".into(), id: usa_id, outer_id: Some(earth_id)},
				"alskdjalgkh  ladhkj EAL ISdh".into(), &store
			),

			BincodeOrganization::create(
				Location {name: "Arizona".into(), id: arizona_id, outer_id: Some(usa_id)},
				" AAA – 44 %%".into(), &store
			),
		).unwrap();

		let start = Instant::now();

		// retrieve `packing` and `eal`
		let results = BincodeOrganization::retrieve(
			&query::Organization
			{
				location: query::Location
				{
					id: Match::HasAny(vec![Borrowed(&earth_id), Borrowed(&usa_id)].into_iter().collect()),
					..Default::default()
				},
				name: MatchStr::Regex(format!("^({}|{})$", packing.name, eal.name)),
				..Default::default()
			},
			&store,
		).await.unwrap();

		println!("\n>>>>> BincodeOrganization::retrieve {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

		// test if `packing` and `eal` were retrieved
		assert!(results.contains(&packing));
		assert!(results.contains(&eal));
		assert!(!results.contains(&aaa));
	}
}

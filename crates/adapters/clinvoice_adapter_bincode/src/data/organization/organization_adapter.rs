use
{
	super::BincodeOrganization,
	crate::util,
	clinvoice_adapter::
	{
		data::{Initializable, MatchWhen, OrganizationAdapter, Updatable},
		DynamicResult, Store,
	},
	clinvoice_data::{Location, Organization, Id},
	std::{fs, io::BufReader},
};

impl<'pass, 'path, 'user> OrganizationAdapter<'pass, 'path, 'user> for BincodeOrganization<'pass, 'path, 'user>
{
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
	fn create(
		location: Location,
		name: &str,
		store: Store<'pass, 'path, 'user>,
	) -> DynamicResult<Self>
	{
		Self::init(&store)?;

		let bincode_organization = Self
		{
			organization: Organization
			{
				id: util::unique_id(&Self::path(&store))?,
				location_id: location.id,
				name: name.into(),
			},
			store,
		};

		bincode_organization.update()?;

		Ok(bincode_organization)
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
	fn retrieve(
		id: MatchWhen<Id>,
		location: MatchWhen<Id>,
		name: MatchWhen<String>,
		store: Store<'pass, 'path, 'user>,
	) -> DynamicResult<Vec<Self>>
	{
		let mut results = Vec::new();

		for node_path in util::read_files(BincodeOrganization::path(&store))?
		{
			let organization: Organization = bincode::deserialize_from(BufReader::new(
				fs::File::open(node_path)?
			))?;

			if id.is_match(&organization.id) &&
				location.is_match(&organization.location_id) &&
				name.is_match(&organization.name)
			{
				results.push(BincodeOrganization {organization, store});
			}
		}

		Ok(results)
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeOrganization, Id, Location, MatchWhen, OrganizationAdapter, util},
		std::{fs, time::Instant},
	};

	#[test]
	fn test_create()
	{
		util::test_temp_store(|store|
		{
			let earth_id = Id::new_v4();
			let usa_id = Id::new_v4();
			let arizona_id = Id::new_v4();
			let phoenix_id = Id::new_v4();
			let some_id = Id::new_v4();

			let start = Instant::now();

			test_create_assertion(BincodeOrganization::create(
				Location {name: "Earth".into(), id: Id::new_v4(), outer_id: None},
				"alsdkjaldkj", *store
			).unwrap());

			test_create_assertion(BincodeOrganization::create(
				Location {name: "USA".into(), id: usa_id, outer_id: Some(earth_id)},
				"alskdjalgkh  ladhkj EAL ISdh", *store
			).unwrap());

			test_create_assertion(BincodeOrganization::create(
				Location {name: "Arizona".into(), id: arizona_id, outer_id: Some(earth_id)},
				" AAA – 44 %%", *store
			).unwrap());

			test_create_assertion(BincodeOrganization::create(
				Location {name: "Phoenix".into(), id: phoenix_id, outer_id: Some(arizona_id)},
				" ^^^ ADSLKJDLASKJD FOCJCI", *store
			).unwrap());

			test_create_assertion(BincodeOrganization::create(
				Location {name: "Some Road".into(), id: some_id, outer_id: Some(phoenix_id)},
				"aldkj doiciuc giguy &&", *store
			).unwrap());

			println!("\n>>>>> BincodeOrganization::create {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 5);
		});
	}

	fn test_create_assertion(bincode_organization: BincodeOrganization)
	{
		let read_result = fs::read(bincode_organization.filepath()).unwrap();
		assert_eq!(bincode_organization.organization, bincode::deserialize(&read_result).unwrap());
	}

	#[test]
	fn test_retrieve()
	{
		util::test_temp_store(|store|
		{
			let earth_id = Id::new_v4();
			let packing = BincodeOrganization::create(
				Location {name: "Earth".into(), id: earth_id, outer_id: None},
				"Packing Co", *store
			).unwrap();

			let usa_id = Id::new_v4();
			let eal = BincodeOrganization::create(
				Location {name: "USA".into(), id: usa_id, outer_id: Some(earth_id)},
				"alskdjalgkh  ladhkj EAL ISdh", *store
			).unwrap();

			let arizona_id = Id::new_v4();
			let aaa = BincodeOrganization::create(
				Location {name: "Arizona".into(), id: arizona_id, outer_id: Some(usa_id)},
				" AAA – 44 %%", *store
			).unwrap();

			let start = Instant::now();
			// retrieve `packing` and `eal`
			let results = BincodeOrganization::retrieve(
				MatchWhen::Any, // id
				MatchWhen::InRange(&|id| id == &earth_id || id == &usa_id), // location
				MatchWhen::HasNone([aaa.organization.name.clone()].iter().collect()), // name
				*store,
			).unwrap();
			println!("\n>>>>> BincodeOrganization::retrieve {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

			// test if `packing` and `eal` were retrieved
			assert!(results.contains(&packing));
			assert!(results.contains(&eal));
			assert!(!results.contains(&aaa));
		});
	}
}

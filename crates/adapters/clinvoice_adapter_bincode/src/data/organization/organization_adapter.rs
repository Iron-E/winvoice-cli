use
{
	super::BincodeOrganization,
	crate::util,
	clinvoice_adapter::
	{
		data::{MatchWhen, OrganizationAdapter, Updatable},
		Store
	},
	clinvoice_data::{Employee, Location, Organization, Id},
	std::{collections::HashSet, error::Error, fs, io::BufReader},
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
	fn create<'name>(
		location: Location,
		name: &'name str,
		representatives: HashSet<Employee>,
		store: Store<'pass, 'path, 'user>,
	) -> Result<Self, Box<dyn Error>>
	{
		Self::init(&store)?;

		let bincode_organization = Self
		{
			organization: Organization
			{
				id: util::unique_id(&Self::path(&store))?,
				location_id: location.id,
				name: name.into(),
				representatives: representatives.iter().map(|rep| rep.id).collect(),
			},
			store,
		};

		bincode_organization.update()?;

		return Ok(bincode_organization);
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store<'pass, 'path, 'user>) -> Result<(), Box<dyn Error>>
	{
		util::create_store_dir(&Self::path(store))?;
		return Ok(());
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
		representatives: MatchWhen<Id>,
		store: Store<'pass, 'path, 'user>,
	) -> Result<HashSet<Self>, Box<dyn Error>>
	{
		let mut results = HashSet::new();

		for node_path in fs::read_dir(BincodeOrganization::path(&store))?.filter_map(
			|node| match node {Ok(n) => Some(n.path()), Err(_) => None}
		)
		{
			let organization: Organization = bincode::deserialize_from(
				BufReader::new(fs::File::open(node_path)?
			))?;

			if id.is_match(&organization.id) &&
				location.is_match(&organization.location_id) &&
				name.is_match(&organization.name) &&
				representatives.set_matches(&organization.representatives)
			{
				results.insert(BincodeOrganization {organization, store});
			}
		}

		return Ok(results);
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeOrganization, HashSet, Id, Location, MatchWhen, OrganizationAdapter, util},
		core::hash::Hash,
		std::{fs, time::Instant},
	};

	#[test]
	fn test_create()
	{
		fn assertion(bincode_organization: BincodeOrganization<'_, '_, '_>)
		{
			let read_result = fs::read(bincode_organization.filepath()).unwrap();
			assert_eq!(bincode_organization.organization, bincode::deserialize(&read_result).unwrap());
		}

		let start = Instant::now();

		util::test_temp_store(|store|
		{
			let earth_id = Id::new_v4();
			assertion(BincodeOrganization::create(
				Location {name: "Earth".into(), id: earth_id, outer_id: None},
				"alsdkjaldkj", HashSet::new(), *store
			).unwrap());

			let usa_id = Id::new_v4();
			assertion(BincodeOrganization::create(
				Location {name: "USA".into(), id: usa_id, outer_id: Some(earth_id)},
				"alskdjalgkh  ladhkj EAL ISdh", HashSet::new(), *store
			).unwrap());

			let arizona_id = Id::new_v4();
			assertion(BincodeOrganization::create(
				Location {name: "Arizona".into(), id: arizona_id, outer_id: Some(earth_id)},
				" AAA – 44 %%", HashSet::new(), *store
			).unwrap());

			let phoenix_id = Id::new_v4();
			assertion(BincodeOrganization::create(
				Location {name: "Phoenix".into(), id: phoenix_id, outer_id: Some(arizona_id)},
				" ^^^ ADSLKJDLASKJD FOCJCI", HashSet::new(), *store
			).unwrap());

			let some_id = Id::new_v4();
			assertion(BincodeOrganization::create(
				Location {name: "Some Road".into(), id: some_id, outer_id: Some(phoenix_id)},
				"aldkj doiciuc giguy &&", HashSet::new(), *store
			).unwrap());

			println!("\n>>>>> BincodeOrganization test_create {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}

	#[test]
	fn test_retrieve()
	{
		fn to_hashset<T>(slice: &[T]) -> HashSet<T> where T : Clone + Eq + Hash
		{
			return slice.into_iter().cloned().collect();
		}

		let start = Instant::now();

		util::test_temp_store(|store|
		{
			let earth_id = Id::new_v4();
			let packing = BincodeOrganization::create(
				Location {name: "Earth".into(), id: earth_id, outer_id: None},
				"Packing Co", HashSet::new(), *store
			).unwrap();

			let usa_id = Id::new_v4();
			let eal = BincodeOrganization::create(
				Location {name: "USA".into(), id: usa_id, outer_id: Some(earth_id)},
				"alskdjalgkh  ladhkj EAL ISdh", HashSet::new(), *store
			).unwrap();

			let arizona_id = Id::new_v4();
			let aaa = BincodeOrganization::create(
				Location {name: "Arizona".into(), id: arizona_id, outer_id: Some(usa_id)},
				" AAA – 44 %%", HashSet::new(), *store
			).unwrap();

			// retrieve `packing` and `eal`
			let results = BincodeOrganization::retrieve(
				MatchWhen::Any, // id
				MatchWhen::InRange(&|id| id == &earth_id || id == &usa_id), // location
				MatchWhen::HasNone(to_hashset(&[aaa.organization.name.clone()])), // name
				MatchWhen::Any, // representatives
				*store,
			).unwrap();

			// test if `packing` and `eal` were retrieved
			assert!(results.contains(&packing));
			assert!(results.contains(&eal));
			assert!(!results.contains(&aaa));

			println!("\n>>>>> BincodeOrganization test_retrieve {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}
}

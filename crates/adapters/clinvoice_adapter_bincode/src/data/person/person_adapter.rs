use
{
	super::BincodePerson,
	crate::
	{
		data::{Error, Result},
		util,
	},

	clinvoice_adapter::
	{
		data::{Error as DataError, Initializable, PersonAdapter, Updatable},
		Store,
	},
	clinvoice_data::Person,
	clinvoice_query as query,
};

#[async_trait::async_trait]
impl PersonAdapter for BincodePerson<'_, '_>
{
	type Error = Error;

	/// # Summary
	///
	/// Create a new [`Person`] on the active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Person`].
	///
	/// # Returns
	///
	/// The newly created [`Person`].
	async fn create(name: String, store: &Store,) -> Result<Person>
	{
		let init_fut = Self::init(&store);

		let person = Person
		{
			id: util::unique_id(&Self::path(&store))?,
			name,
		};

		init_fut.await?;
		BincodePerson {person: &person, store}.update().await?;

		Ok(person)
	}

	/// # Summary
	///
	/// Retrieve some [`Person`] from the active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Person`].
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`Job`]s.
	async fn retrieve(query: &query::Person, store: &Store) -> Result<Vec<Person>>
	{
		Self::init(&store).await?;

		util::retrieve(Self::path(store),
			|p| query.matches(p).map_err(|e| DataError::from(e).into())
		).await
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		std::{borrow::Cow::Borrowed, time::Instant},

		super::{BincodePerson, Person, PersonAdapter, query, Store, util},

		clinvoice_query::{Match, MatchStr},

		tokio::fs,
	};

	#[tokio::test]
	async fn create()
	{
		let store = util::temp_store();

		let start = Instant::now();

		let (widdle, long, steven, jingle_bob, asldkj_jdsoai) = futures::try_join!(
			BincodePerson::create("Widdle".into(), &store),
			BincodePerson::create("Long".into(), &store),
			BincodePerson::create("Steven".into(), &store),
			BincodePerson::create("JingleBob".into(), &store),
			BincodePerson::create("asldkj jdsoai".into(), &store),
		).unwrap();

		println!("\n>>>>> BincodePerson::create {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 5);

		futures::join!(
			create_assertion(widdle, &store),
			create_assertion(long, &store),
			create_assertion(steven, &store),
			create_assertion(jingle_bob, &store),
			create_assertion(asldkj_jdsoai, &store),
		);
	}

	async fn create_assertion(person: Person, store: &Store)
	{
		let read_result = fs::read(BincodePerson {person: &person, store}.filepath()).await.unwrap();
		assert_eq!(person, bincode::deserialize(&read_result).unwrap());
	}

	#[tokio::test]
	async fn retrieve()
	{
		let store = util::temp_store();

		let (flingo, bob, slimdi, longone) = futures::try_join!(
			BincodePerson::create("flingo".into(), &store),
			BincodePerson::create("bob".into(), &store),
			BincodePerson::create("slimdi".into(), &store),
			BincodePerson::create("longone".into(), &store),
		).unwrap();

		let start = Instant::now();

		let (only_bob, longone_slimdi) = futures::try_join!(
			// Retrieve bob
			BincodePerson::retrieve(
				&query::Person
				{
					id: Match::EqualTo(Borrowed(&bob.id)),
					..Default::default()
				},
				&store,
			),

			// Retrieve longone and slimdi
			BincodePerson::retrieve(
				&query::Person
				{
					name: MatchStr::Regex(format!("^({}|{})$", longone.name, slimdi.name)),
					..Default::default()
				},
				&store,
			),
		).unwrap();

		println!("\n>>>>> BincodePerson::retrieve {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 2);

		// Assert bob is the only one retrieved
		assert!(!only_bob.contains(&flingo));
		assert!(only_bob.contains(&bob));
		assert!(!only_bob.contains(&slimdi));
		assert!(!only_bob.contains(&longone));

		// Assert bob is the only one retrieved
		assert!(!longone_slimdi.contains(&flingo));
		assert!(!longone_slimdi.contains(&bob));
		assert!(longone_slimdi.contains(&slimdi));
		assert!(longone_slimdi.contains(&longone));
	}
}

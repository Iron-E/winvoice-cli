use clinvoice_adapter::{
	data::{Error as DataError, Initializable, LocationAdapter, Updatable},
	Store,
};
use clinvoice_data::Location;
use clinvoice_query as query;

use super::BincodeLocation;
use crate::{
	data::{Error, Result},
	util,
};

impl LocationAdapter for BincodeLocation<'_, '_>
{
	type Error = Error;

	/// # Summary
	///
	/// Create a new `Location` with a generated ID.
	///
	/// # Parameters
	///
	/// See [`Location`].
	///
	/// # Returns
	///
	/// ```ignore
	/// Location {name, id: /* generated */};
	/// ```
	fn create(name: String, store: &Store) -> Result<Location>
	{
		Self::init(&store)?;

		let location = Location {
			id: util::unique_id(&Self::path(&store))?,
			name,
			outer_id: None,
		};

		BincodeLocation {
			location: &location,
			store,
		}
		.update()?;

		Ok(location)
	}

	/// # Summary
	///
	/// Create a new [`Location`] which is inside of `self`.
	///
	/// # Parameters
	///
	/// See [`Location`].
	///
	/// # Returns
	///
	/// ```ignore
	/// Location {name, id: /* generated */, outside_id: self.unroll().id};
	/// ```
	fn create_inner(&self, name: String) -> Result<Location>
	{
		let inner_location = Location {
			id: util::unique_id(&Self::path(&self.store))?,
			name,
			outer_id: Some(self.location.id),
		};

		BincodeLocation {
			location: &inner_location,
			store:    self.store,
		}
		.update()?;

		Ok(inner_location)
	}

	/// # Summary
	///
	/// Retrieve a [`Location`] from an active [`Store`](core::Store).
	///
	/// # Parameters
	///
	/// See [`Location`].
	///
	/// # Returns
	///
	/// * An [`Error`], when something goes wrong.
	/// * A list of matches, if there are any.
	fn retrieve(query: &query::Location, store: &Store) -> Result<Vec<Location>>
	{
		Self::init(&store)?;

		util::retrieve(Self::path(store), |l| {
			query.matches(l).map_err(|e| DataError::from(e).into())
		})
	}
}

#[cfg(test)]
mod tests
{
	use std::{borrow::Cow::Borrowed, fs, time::Instant};

	use clinvoice_query::Match;

	use super::{query, util, BincodeLocation, Location, LocationAdapter, Store};

	#[test]
	fn create()
	{
		util::temp_store(|store| {
			let start = Instant::now();
			let earth = BincodeLocation::create("Earth".into(), &store).unwrap();
			let usa = BincodeLocation {
				location: &earth,
				store,
			}
			.create_inner("USA".into())
			.unwrap();
			let arizona = BincodeLocation {
				location: &usa,
				store,
			}
			.create_inner("Arizona".into())
			.unwrap();
			let phoenix = BincodeLocation {
				location: &arizona,
				store,
			}
			.create_inner("Phoenix".into())
			.unwrap();
			println!(
				"\n>>>>> BincodeLocation::start {}us <<<<<\n",
				Instant::now().duration_since(start).as_micros() / 4
			);

			assert_eq!(usa.outer_id, Some(earth.id));
			assert_eq!(arizona.outer_id, Some(usa.id));
			assert_eq!(phoenix.outer_id, Some(arizona.id));
			create_assertion(earth, &store);
			create_assertion(usa, &store);
			create_assertion(arizona, &store);
			create_assertion(phoenix, &store);
		});
	}

	/// The assertion most commonly used for the [`create` test](test_create).
	fn create_assertion(location: Location, store: &Store)
	{
		let read_result = fs::read(
			BincodeLocation {
				location: &location,
				store,
			}
			.filepath(),
		)
		.unwrap();
		assert_eq!(location, bincode::deserialize(&read_result).unwrap());
	}

	#[test]
	fn retrieve()
	{
		util::temp_store(|store| {
			let earth = BincodeLocation::create("Earth".into(), &store).unwrap();
			let usa = BincodeLocation {
				location: &earth,
				store,
			}
			.create_inner("USA".into())
			.unwrap();
			let arizona = BincodeLocation {
				location: &usa,
				store,
			}
			.create_inner("Arizona".into())
			.unwrap();
			let phoenix = BincodeLocation {
				location: &arizona,
				store,
			}
			.create_inner("Phoenix".into())
			.unwrap();

			let start = Instant::now();

			// Retrieve everything.
			let everything = BincodeLocation::retrieve(&Default::default(), &store).unwrap();

			// Retrieve Arizona
			let only_arizona = BincodeLocation::retrieve(
				&query::Location {
					id: Match::HasAny(
						vec![Borrowed(&earth.id), Borrowed(&arizona.id)]
							.into_iter()
							.collect(),
					),
					outer: query::OuterLocation::Some(query::Location::default().into()),
					..Default::default()
				},
				&store,
			)
			.unwrap();

			println!(
				"\n>>>>> BincodeLocation::retrieve {}us <<<<<\n",
				Instant::now().duration_since(start).as_micros() / 2
			);

			// Assert the results contains all values
			assert!(everything.contains(&earth));
			assert!(everything.contains(&usa));
			assert!(everything.contains(&arizona));
			assert!(everything.contains(&phoenix));

			// Assert the results contains all values
			assert!(!only_arizona.contains(&earth));
			assert!(!only_arizona.contains(&usa));
			assert!(only_arizona.contains(&arizona));
			assert!(!only_arizona.contains(&phoenix));
		})
	}
}

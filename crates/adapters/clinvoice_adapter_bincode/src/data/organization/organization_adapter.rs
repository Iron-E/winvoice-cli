use clinvoice_adapter::{
	data::{Error as DataError, Initializable, OrganizationAdapter, Updatable},
	Store,
};
use clinvoice_data::{Location, Organization};
use clinvoice_query as query;

use super::BincodeOrganization;
use crate::{
	data::{Error, Result},
	util,
};

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
	fn create(location: Location, name: String, store: &Store) -> Result<Organization>
	{
		Self::init(&store)?;

		let organization = Organization {
			id: util::unique_id(&Self::path(&store))?,
			location_id: location.id,
			name,
		};

		BincodeOrganization {
			organization: &organization,
			store,
		}
		.update()?;

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
	fn retrieve(query: &query::Organization, store: &Store) -> Result<Vec<Organization>>
	{
		Self::init(&store)?;

		util::retrieve(Self::path(store), |o| {
			query.matches(o).map_err(|e| DataError::from(e).into())
		})
	}
}

#[cfg(test)]
mod tests
{
	use std::{borrow::Cow::Borrowed, fs, time::Instant};

	use clinvoice_data::Id;
	use clinvoice_query::{Match, MatchStr};

	use super::{
		query,
		util,
		BincodeOrganization,
		Location,
		Organization,
		OrganizationAdapter,
		Store,
	};

	#[test]
	fn create()
	{
		util::temp_store(|store| {
			let earth_id = Id::new_v4();
			let usa_id = Id::new_v4();
			let arizona_id = Id::new_v4();
			let phoenix_id = Id::new_v4();
			let some_id = Id::new_v4();

			let start = Instant::now();

			create_assertion(
				BincodeOrganization::create(
					Location {
						name: "Earth".into(),
						id: Id::new_v4(),
						outer_id: None,
					},
					"alsdkjaldkj".into(),
					&store,
				)
				.unwrap(),
				&store,
			);

			create_assertion(
				BincodeOrganization::create(
					Location {
						name: "USA".into(),
						id: usa_id,
						outer_id: Some(earth_id),
					},
					"alskdjalgkh  ladhkj EAL ISdh".into(),
					&store,
				)
				.unwrap(),
				&store,
			);

			create_assertion(
				BincodeOrganization::create(
					Location {
						name: "Arizona".into(),
						id: arizona_id,
						outer_id: Some(earth_id),
					},
					" AAA – 44 %%".into(),
					&store,
				)
				.unwrap(),
				&store,
			);

			create_assertion(
				BincodeOrganization::create(
					Location {
						name: "Phoenix".into(),
						id: phoenix_id,
						outer_id: Some(arizona_id),
					},
					" ^^^ ADSLKJDLASKJD FOCJCI".into(),
					&store,
				)
				.unwrap(),
				&store,
			);

			create_assertion(
				BincodeOrganization::create(
					Location {
						name: "Some Road".into(),
						id: some_id,
						outer_id: Some(phoenix_id),
					},
					"aldkj doiciuc giguy &&".into(),
					&store,
				)
				.unwrap(),
				&store,
			);

			println!(
				"\n>>>>> BincodeOrganization::create {}us <<<<<\n",
				Instant::now().duration_since(start).as_micros() / 5
			);
		});
	}

	fn create_assertion(organization: Organization, store: &Store)
	{
		let read_result = fs::read(
			BincodeOrganization {
				organization: &organization,
				store,
			}
			.filepath(),
		)
		.unwrap();
		assert_eq!(organization, bincode::deserialize(&read_result).unwrap());
	}

	#[test]
	fn retrieve()
	{
		util::temp_store(|store| {
			let earth_id = Id::new_v4();
			let packing = BincodeOrganization::create(
				Location {
					name: "Earth".into(),
					id: earth_id,
					outer_id: None,
				},
				"Packing Co".into(),
				&store,
			)
			.unwrap();

			let usa_id = Id::new_v4();
			let eal = BincodeOrganization::create(
				Location {
					name: "USA".into(),
					id: usa_id,
					outer_id: Some(earth_id),
				},
				"alskdjalgkh  ladhkj EAL ISdh".into(),
				&store,
			)
			.unwrap();

			let arizona_id = Id::new_v4();
			let aaa = BincodeOrganization::create(
				Location {
					name: "Arizona".into(),
					id: arizona_id,
					outer_id: Some(usa_id),
				},
				" AAA – 44 %%".into(),
				&store,
			)
			.unwrap();

			let start = Instant::now();

			// retrieve `packing` and `eal`
			let results = BincodeOrganization::retrieve(
				&query::Organization {
					location: query::Location {
						id: Match::HasAny(
							vec![Borrowed(&earth_id), Borrowed(&usa_id)]
								.into_iter()
								.collect(),
						),
						..Default::default()
					},
					name: MatchStr::Regex(format!("^({}|{})$", packing.name, eal.name)),
					..Default::default()
				},
				&store,
			)
			.unwrap();
			println!(
				"\n>>>>> BincodeOrganization::retrieve {}us <<<<<\n",
				Instant::now().duration_since(start).as_micros()
			);

			// test if `packing` and `eal` were retrieved
			assert!(results.contains(&packing));
			assert!(results.contains(&eal));
			assert!(!results.contains(&aaa));
		});
	}
}

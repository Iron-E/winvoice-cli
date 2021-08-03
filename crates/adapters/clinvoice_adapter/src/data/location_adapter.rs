#![allow(clippy::wrong_self_convention)]

use
{
	std::{borrow::Cow::Borrowed, error::Error, marker::Send},
	super::{Deletable, Initializable, Updatable},
	crate::Store,

	clinvoice_data::{Location, views::LocationView},
	clinvoice_query as query,

	futures::{FutureExt, TryFutureExt},
};

#[async_trait::async_trait]
pub trait LocationAdapter :
	Deletable<Error = <Self as LocationAdapter>::Error> +
	Initializable<Error = <Self as LocationAdapter>::Error> +
	Updatable<Error = <Self as LocationAdapter>::Error> +
{
	type Error : From<super::Error> + Error;

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
	async fn create(name: String, store: &Store) -> Result<Location, <Self as LocationAdapter>::Error>;

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
	async fn create_inner(&self, name: String) -> Result<Location, <Self as LocationAdapter>::Error>;

	/// # Summary
	///
	/// Convert some `location` into a [`LocationView`].
	async fn into_view(location: Location, store: &Store) -> Result<LocationView, <Self as LocationAdapter>::Error> where
		Self : Send,
	{
		let outer_location_fut = Self::outers(&location, store).map_ok(|mut outers|
		{
			outers.reverse();
			outers.into_iter().fold(None::<LocationView>, |previous, outer_location| Some(LocationView
			{
				id: outer_location.id,
				name: outer_location.name,
				outer: previous.map(|l| l.into()),
			})).map(|l| Box::new(l))
		});

		Ok(LocationView
		{
			id: location.id,
			name: location.name.clone(),
			outer: outer_location_fut.await?,
		})
	}

	/// # Summary
	///
	/// Get the [`Location`]s which contain this [`Location`].
	async fn outers(location: &Location, store: &Store) -> Result<Vec<Location>, <Self as LocationAdapter>::Error>
	{
		let mut outer_locations = Vec::new();
		let mut outer_id = location.outer_id;

		while let Some(id) = outer_id
		{
			let query = query::Location
			{
				id: query::Match::EqualTo(Borrowed(&id)),
				..Default::default()
			};

			Self::retrieve(&query, &store).map(|result| result.and_then(|retrieved|
				retrieved.into_iter().next().map(|adapted_location|
				{
					outer_id = adapted_location.outer_id;
					outer_locations.push(adapted_location);
				}).ok_or_else(|| super::Error::DataIntegrity(id).into())
			)).await?;
		}

		Ok(outer_locations)
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
	async fn retrieve(
		query: &query::Location,
		store: &Store,
	) -> Result<Vec<Location>, <Self as LocationAdapter>::Error>;
}

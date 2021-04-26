use
{
	std::{borrow::Cow::Borrowed, error::Error},
	super::{Deletable, Initializable, Updatable},
	crate::Store,

	clinvoice_data::{Location, views::LocationView},
	clinvoice_query as query,
};

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
	fn create(name: &str, store: &Store) -> Result<Location, <Self as LocationAdapter>::Error>;

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
	fn create_inner(&self, name: &str) -> Result<Location, <Self as LocationAdapter>::Error>;

	/// # Summary
	///
	/// Convert some `location` into a [`LocationView`].
	fn into_view(location: Location, store: &Store) -> Result<LocationView, <Self as LocationAdapter>::Error>
	{
		let mut outer_locations = Self::outers(&location, store)?;
		outer_locations.reverse();

		Ok(LocationView
		{
			id: location.id,
			name: location.name,
			outer: outer_locations.into_iter().fold(None,
				|previous: Option<LocationView>, outer_location| Some(LocationView
				{
					id: outer_location.id,
					name: outer_location.name,
					outer: previous.map(|l| l.into()),
				}),
			).map(|l| l.into()),
		})
	}

	/// # Summary
	///
	/// Get the [`Location`]s which contain this [`Location`].
	fn outers(location: &Location, store: &Store) -> Result<Vec<Location>, super::Error>
	{
		let mut outer_locations = Vec::<Location>::new();

		let mut outer_id = location.outer_id;
		while let Some(id) = outer_id
		{
			if let Ok(results) = Self::retrieve(
				&query::Location
				{
					id: query::Match::EqualTo(Borrowed(&id)),
					..Default::default()
				},
				&store,
			)
			{
				if let Some(adapted_location) = results.into_iter().next()
				{
					outer_id = adapted_location.outer_id;
					outer_locations.push(adapted_location);
					continue;
				}
			}

			return Err(super::Error::DataIntegrity(id));
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
	fn retrieve(
		query: &query::Location,
		store: &Store,
	) -> Result<Vec<Location>, <Self as LocationAdapter>::Error>;
}

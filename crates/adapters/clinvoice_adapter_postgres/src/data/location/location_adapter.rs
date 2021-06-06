use
{
	super::PostgresLocation,
	crate::data::{Error, Result},

	clinvoice_adapter::
	{
		data::{Error as DataError, Initializable, LocationAdapter, Updatable},
		Store,
	},
	clinvoice_data::Location,
	clinvoice_query as query,
};

impl LocationAdapter for PostgresLocation<'_, '_>
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
		todo!()
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
		todo!()
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
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	#[test]
	fn create()
	{
	}

	#[test]
	fn retrieve()
	{
	}
}
